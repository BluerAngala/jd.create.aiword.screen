/**
 * 卡密系统相关函数（含加解密）
 */
const crypto = require("crypto");

const card = {};

// ==================== AES-256-GCM 加解密配置 ====================
const NONCE_LENGTH = 12;
const AUTH_TAG_LENGTH = 16;

/// 共享加密密钥（32 字节 = 256 位）
/// 重要：客户端和服务端必须使用相同的密钥！
const DEFAULT_KEY = "5ee88f388e79950a48e7f84f42676d5fa9701549844354427374f20cf1e35d63";

/**
 * 获取加密密钥
 */
card.getEncryptionKey = function () {
  const envKey = process.env.ENCRYPTION_KEY;
  if (envKey) {
    if (envKey.length === 64 && /^[0-9a-fA-F]+$/.test(envKey)) {
      return Buffer.from(envKey, "hex");
    }
    return Buffer.from(envKey, "utf8");
  }
  // 默认密钥是 hex 格式
  return Buffer.from(DEFAULT_KEY, "hex");
};

/**
 * AES-256-GCM 加密
 */
card.encrypt = function (plaintext) {
  const key = card.getEncryptionKey();
  const nonce = crypto.randomBytes(NONCE_LENGTH);
  const cipher = crypto.createCipheriv("aes-256-gcm", key, nonce);
  const encrypted = Buffer.concat([cipher.update(plaintext, "utf8"), cipher.final()]);
  const authTag = cipher.getAuthTag();
  return Buffer.concat([nonce, encrypted, authTag]).toString("base64");
};

/**
 * AES-256-GCM 解密
 */
card.decrypt = function (ciphertextB64) {
  const key = card.getEncryptionKey();
  const combined = Buffer.from(ciphertextB64, "base64");
  if (combined.length < NONCE_LENGTH + AUTH_TAG_LENGTH) {
    throw new Error("密文格式错误：长度不足");
  }
  const nonce = combined.subarray(0, NONCE_LENGTH);
  const authTag = combined.subarray(combined.length - AUTH_TAG_LENGTH);
  const ciphertext = combined.subarray(NONCE_LENGTH, combined.length - AUTH_TAG_LENGTH);
  const decipher = crypto.createDecipheriv("aes-256-gcm", key, nonce);
  decipher.setAuthTag(authTag);
  try {
    return Buffer.concat([decipher.update(ciphertext), decipher.final()]).toString("utf8");
  } catch (err) {
    throw new Error("解密失败：数据可能被篡改");
  }
};

/**
 * 解密请求数据
 */
card.decryptRequest = function (data) {
  if (!data.encrypted_body) return data;
  return JSON.parse(card.decrypt(data.encrypted_body));
};

/**
 * 加密响应数据
 */
card.encryptResponse = function (response) {
  return { encrypted_body: card.encrypt(JSON.stringify(response)) };
};

/**
 * 检查是否为加密请求
 */
card.isEncryptedRequest = function (data) {
  return !!(data && data.encrypted_body);
};

// ==================== 卡密工具函数 ====================

/**
 * 生成随机字符串
 */
card.generateRandomString = function (length) {
  return crypto
    .randomBytes(Math.ceil(length / 2))
    .toString("hex")
    .slice(0, length);
};

/**
 * 生成唯一密钥
 */
card.generateUniqueKeys = function (num, length) {
  const keys = new Set();
  while (keys.size < num) {
    keys.add(card.generateRandomString(length));
  }
  return Array.from(keys);
};

/**
 * 极简卡密校验函数
 */
card.validateCardKey = function (record, machineCode = null, user_id = null) {
  const updateData = {};

  // 用户ID校验
  if (user_id && record.buy_user_id) {
    if (user_id !== record.buy_user_id) {
      return { valid: false, message: "此卡密不属于您，无法使用" };
    }
  }

  // 时间校验
  if (record.limit_days !== -1) {
    const now = Date.now();
    if (record.activate_time === 0) {
      updateData.activate_time = now;
      updateData.expire_time = now + record.limit_days * 24 * 60 * 60 * 1000;
    } else if (now > record.expire_time) {
      return { valid: false, message: "卡密已过期" };
    }
  }

  // 次数校验
  if (record.total_times !== -1) {
    if (record.remaining_times <= 0) {
      return { valid: false, message: "使用次数已用完" };
    }
    updateData.used_times = record.used_times + 1;
    updateData.remaining_times = record.remaining_times - 1;
  }

  // 机器码校验
  if (record.max_machine_count !== -1) {
    if (!machineCode) {
      return { valid: false, message: "此卡密需要提供机器码" };
    }
    const machines = record.authorized_machines || [];
    if (!machines.includes(machineCode)) {
      if (machines.length >= record.max_machine_count) {
        return { valid: false, message: "已达到最大授权机器数量" };
      }
      updateData.authorized_machines = [...machines, machineCode];
    }
  }

  return {
    valid: true,
    message: "校验成功",
    updateData: Object.keys(updateData).length > 0 ? updateData : null,
  };
};

/**
 * 导出卡密到CSV
 */
card.exportCardsToCSV = async function (ids, db) {
  if (!Array.isArray(ids) || ids.length === 0) {
    return { csvContent: null, count: 0, message: "无有效的卡密ID" };
  }

  const $ = db.command;
  const res = await db
    .collection("vk-card-key")
    .where({ _id: $.in(ids) })
    .field({
      _id: true,
      card_code: true,
      limit_days: true,
      max_machine_count: true,
    })
    .get();

  const rows = res.data || [];
  const columns = [
    { key: "_id", label: "卡号" },
    { key: "card_code", label: "密钥" },
    { key: "limit_days", label: "有效期（天）" },
    { key: "max_machine_count", label: "最大机器数" },
  ];

  const headers = columns.map((c) => c.label);
  const esc = (v) => {
    if (v === undefined || v === null) return "";
    const s = String(v).replace(/"/g, '""');
    return `"${s}"`;
  };

  const csvRows = rows.map((r) => columns.map((c) => esc(r[c.key])).join(","));
  const csv = [headers.join(","), ...csvRows].join("\r\n");

  return { csvContent: csv, count: rows.length, message: "CSV 导出完成" };
};

/**
 * 通用参数获取函数 - 兼容函数参数和HTTP请求参数
 */
card.getParams = function (funcParams, httpInfo) {
  let httpParams = {};
  if (httpInfo && httpInfo.body) {
    try {
      httpParams = typeof httpInfo.body === "string" ? JSON.parse(httpInfo.body) : httpInfo.body;
    } catch (error) {
      console.error("解析HTTP请求体失败:", error);
      httpParams = {};
    }
  }

  const safeFuncParams =
    funcParams && typeof funcParams === "object"
      ? Object.fromEntries(Object.entries(funcParams).filter(([, v]) => v !== undefined))
      : {};

  if (Object.keys(safeFuncParams).length === 0) {
    return httpParams || {};
  }

  return Object.assign({}, httpParams || {}, safeFuncParams);
};

module.exports = card;
