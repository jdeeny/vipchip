
use std::collections::HashMap;

pub fn examples() -> HashMap<&'static str, Vec<u8>> {
    let mut programs = HashMap::new();

    programs.insert("moveguy",
                    vec![0xC3, 0x1F, 0xC4, 0x0F, 0xA2, 0x30, 0xD3, 0x48, 0xD3, 0x48, 0x60, 0x05,
                         0xE0, 0xA1, 0x74, 0xFF, 0x60, 0x08, 0xE0, 0xA1, 0x74, 0x01, 0x60, 0x07,
                         0xE0, 0xA1, 0x73, 0xFF, 0x60, 0x09, 0xE0, 0xA1, 0x73, 0x01, 0xD3, 0x48,
                         0xFF, 0x07, 0x3F, 0x00, 0x12, 0x24, 0x6F, 0x03, 0xFF, 0x15, 0x12, 0x08,
                         0x70, 0x70, 0x20, 0x70, 0xA8, 0x20, 0x50, 0x50]);

    programs.insert("monitor",
                    vec![0x60, 0x10, 0x61, 0x04, 0xA2, 0xA2, 0xD0, 0x1B, 0x70, 0x08, 0xA2, 0xB8,
                         0xD0, 0x13, 0x70, 0x08, 0xD0, 0x13, 0x70, 0x08, 0xA2, 0xAD, 0xD0, 0x1B,
                         0x60, 0x10, 0x71, 0x0B, 0xA2, 0xBB, 0xD0, 0x1F, 0x70, 0x08, 0x71, 0x07,
                         0xA2, 0xD9, 0xD0, 0x18, 0x70, 0x08, 0xD0, 0x18, 0x70, 0x08, 0x71, 0xF9,
                         0xA2, 0xCA, 0xD0, 0x1F, 0x6A, 0x18, 0x6B, 0x20, 0x6C, 0x07, 0x69, 0x00,
                         0x68, 0x0F, 0x22, 0x54, 0x22, 0x54, 0x79, 0x01, 0x89, 0x82, 0x22, 0x54,
                         0x6F, 0x04, 0xFF, 0x15, 0xFF, 0x07, 0x3F, 0x00, 0x12, 0x4C, 0x12, 0x40,
                         0xA2, 0x62, 0xF9, 0x1E, 0xDA, 0xCF, 0xA2, 0x82, 0xF9, 0x1E, 0xDB, 0xCF,
                         0x00, 0xEE, 0x0F, 0x30, 0x7C, 0x7C, 0xF8, 0xF4, 0xE0, 0xE8, 0xF0, 0xE8,
                         0xE0, 0x68, 0x70, 0x34, 0x08, 0x00, 0x0F, 0x30, 0x7C, 0x7C, 0xF8, 0xF4,
                         0xE0, 0xE8, 0xF0, 0xE8, 0xE0, 0x68, 0x70, 0x34, 0x08, 0x00, 0xF0, 0x0C,
                         0x46, 0x66, 0x33, 0x13, 0x0B, 0x0B, 0x1F, 0x0F, 0x0F, 0x1E, 0x1E, 0x1C,
                         0x30, 0x00, 0xF0, 0x0C, 0x46, 0x66, 0x33, 0x13, 0x0B, 0x0B, 0x1F, 0x0F,
                         0x0F, 0x1E, 0x1E, 0x1C, 0x30, 0x00, 0x3F, 0x3F, 0x3F, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0x3C, 0xFC, 0xFC, 0xFC, 0x3C, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0xFF, 0xFF, 0xFF, 0x3C, 0x3C, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3F, 0x3F, 0x3F, 0x00, 0x07, 0x1C, 0x73, 0x7F, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0x3C, 0xFC, 0xFC, 0xFC, 0x00, 0xE0, 0xD8, 0x26,
                         0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0x33, 0xCC, 0x33, 0xFF]);
    programs.insert("fastmonitor",
                    vec![0x60, 0x10, 0x61, 0x04, 0xA2, 0xA2, 0xD0, 0x1B, 0x70, 0x08, 0xA2, 0xB8,
                         0xD0, 0x13, 0x70, 0x08, 0xD0, 0x13, 0x70, 0x08, 0xA2, 0xAD, 0xD0, 0x1B,
                         0x60, 0x10, 0x71, 0x0B, 0xA2, 0xBB, 0xD0, 0x1F, 0x70, 0x08, 0x71, 0x07,
                         0xA2, 0xD9, 0xD0, 0x18, 0x70, 0x08, 0xD0, 0x18, 0x70, 0x08, 0x71, 0xF9,
                         0xA2, 0xCA, 0xD0, 0x1F, 0x6A, 0x18, 0x6B, 0x20, 0x6C, 0x07, 0x69, 0x00,
                         0x68, 0x0F, 0x22, 0x54, 0x22, 0x54, 0x79, 0x01, 0x89, 0x82, 0x22, 0x54,
                         0xFF, 0x07, 0x3F, 0x00, 0x12, 0x48, 0x6F, 0x01, 0xFF, 0x15, 0x12, 0x40,
                         0xA2, 0x62, 0xF9, 0x1E, 0xDA, 0xCF, 0xA2, 0x82, 0xF9, 0x1E, 0xDB, 0xCF,
                         0x00, 0xEE, 0x0F, 0x30, 0x7C, 0x7C, 0xF8, 0xF4, 0xE0, 0xE8, 0xF0, 0xE8,
                         0xE0, 0x68, 0x70, 0x34, 0x08, 0x00, 0x0F, 0x30, 0x7C, 0x7C, 0xF8, 0xF4,
                         0xE0, 0xE8, 0xF0, 0xE8, 0xE0, 0x68, 0x70, 0x34, 0x08, 0x00, 0xF0, 0x0C,
                         0x46, 0x66, 0x33, 0x13, 0x0B, 0x0B, 0x1F, 0x0F, 0x0F, 0x1E, 0x1E, 0x1C,
                         0x30, 0x00, 0xF0, 0x0C, 0x46, 0x66, 0x33, 0x13, 0x0B, 0x0B, 0x1F, 0x0F,
                         0x0F, 0x1E, 0x1E, 0x1C, 0x30, 0x00, 0x3F, 0x3F, 0x3F, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0x3C, 0xFC, 0xFC, 0xFC, 0x3C, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0xFF, 0xFF, 0xFF, 0x3C, 0x3C, 0x3C, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3F, 0x3F, 0x3F, 0x00, 0x07, 0x1C, 0x73, 0x7F, 0x3C, 0x3C,
                         0x3C, 0x3C, 0x3C, 0x3C, 0x3C, 0xFC, 0xFC, 0xFC, 0x00, 0xE0, 0xD8, 0x26,
                         0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0x33, 0xCC, 0x33, 0xFF]);

    programs.insert("outlaw",
                    vec![0x13, 0xB4, 0x38, 0x38, 0x3B, 0x1B, 0x1B, 0x1B, 0xDF, 0xDE, 0xD8, 0xD8,
                         0xF8, 0x78, 0x18, 0x1C, 0x0C, 0x00, 0x00, 0x70, 0xF8, 0x70, 0x67, 0x7C,
                         0x60, 0x60, 0x78, 0x28, 0xEC, 0x18, 0x3E, 0x1C, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x66, 0xC3, 0x18, 0x3E, 0x1C, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x24, 0x36, 0x00, 0x00, 0x06, 0x1F, 0x0E, 0xE6, 0x3E,
                         0x06, 0x06, 0x1E, 0x14, 0x37, 0x18, 0x7C, 0x38, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x66, 0xC3, 0x18, 0x7C, 0x38, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x24, 0x6C, 0xFF, 0xFF, 0x80, 0x60, 0x06, 0xE0, 0x9E,
                         0x00, 0xEE, 0x69, 0x01, 0x60, 0x05, 0xE0, 0xA1, 0x69, 0x02, 0x60, 0x08,
                         0xE0, 0xA1, 0x69, 0x03, 0x8B, 0xE0, 0x7B, 0x09, 0x8A, 0xD0, 0x7A, 0x05,
                         0xA2, 0x5B, 0xDB, 0xA1, 0x00, 0xEE, 0xDB, 0xA1, 0x7B, 0x01, 0x49, 0x02,
                         0x7A, 0xFF, 0x49, 0x03, 0x7A, 0x01, 0x4A, 0x01, 0x69, 0x03, 0x4A, 0x1E,
                         0x69, 0x02, 0x4B, 0x3F, 0x69, 0x00, 0x49, 0x00, 0x00, 0xEE, 0xDB, 0xA1,
                         0x4F, 0x00, 0x00, 0xEE, 0x69, 0x00, 0x00, 0xEE, 0x33, 0x00, 0x00, 0xEE,
                         0x63, 0x01, 0xC0, 0x03, 0x40, 0x01, 0x63, 0x02, 0x40, 0x02, 0x63, 0x03,
                         0x85, 0x80, 0x75, 0xFF, 0x84, 0x70, 0x74, 0x05, 0xA2, 0x5B, 0xD5, 0x41,
                         0x00, 0xEE, 0xD5, 0x41, 0x75, 0xFF, 0x43, 0x02, 0x74, 0xFF, 0x43, 0x03,
                         0x74, 0x01, 0x44, 0x01, 0x63, 0x03, 0x44, 0x1E, 0x63, 0x02, 0x45, 0x00,
                         0x63, 0x00, 0x43, 0x00, 0x00, 0xEE, 0xD5, 0x41, 0x4F, 0x00, 0x00, 0xEE,
                         0x63, 0x00, 0x00, 0xEE, 0x81, 0xE0, 0x82, 0xD0, 0x60, 0x07, 0xE0, 0xA1,
                         0x71, 0xFF, 0x60, 0x09, 0xE0, 0xA1, 0x71, 0x01, 0x60, 0x05, 0xE0, 0xA1,
                         0x72, 0xFF, 0x60, 0x08, 0xE0, 0xA1, 0x72, 0x01, 0x60, 0x00, 0x51, 0xE0,
                         0x60, 0x18, 0x52, 0xD0, 0x60, 0x18, 0x4C, 0x18, 0x60, 0x0C, 0x6F, 0x06,
                         0xEF, 0xA1, 0x60, 0x00, 0x41, 0x00, 0x61, 0x01, 0x41, 0x15, 0x61, 0x14,
                         0x42, 0x00, 0x62, 0x01, 0x42, 0x12, 0x62, 0x11, 0xA2, 0x11, 0xFC, 0x1E,
                         0xDE, 0xDC, 0x8E, 0x10, 0x8D, 0x20, 0x8C, 0x00, 0xA2, 0x11, 0xFC, 0x1E,
                         0xDE, 0xDC, 0x00, 0xEE, 0x71, 0xFF, 0x00, 0xEE, 0x71, 0x01, 0x00, 0xEE,
                         0x72, 0xFF, 0x00, 0xEE, 0x72, 0xFF, 0x00, 0xEE, 0x72, 0x01, 0x00, 0xEE,
                         0x72, 0x01, 0x00, 0xEE, 0x80, 0x00, 0x00, 0xEE, 0x22, 0xA4, 0x00, 0xEE,
                         0xC0, 0x1C, 0xB3, 0x3C, 0x81, 0x80, 0x82, 0x70, 0x23, 0x5C, 0x60, 0x00,
                         0x51, 0x80, 0x60, 0x18, 0x52, 0x70, 0x60, 0x18, 0x46, 0x18, 0x60, 0x0C,
                         0x41, 0x23, 0x61, 0x24, 0x41, 0x38, 0x61, 0x37, 0x42, 0x00, 0x62, 0x01,
                         0x42, 0x12, 0x62, 0x11, 0xA2, 0x35, 0xF6, 0x1E, 0xD8, 0x7C, 0x88, 0x10,
                         0x87, 0x20, 0x86, 0x00, 0xA2, 0x35, 0xF6, 0x1E, 0xD8, 0x7C, 0x00, 0xEE,
                         0xD8, 0x7C, 0xA2, 0x5B, 0xDB, 0xA1, 0x60, 0x20, 0xF0, 0x18, 0xF0, 0x15,
                         0xF0, 0x07, 0x30, 0x00, 0x13, 0xA4, 0x13, 0xB4, 0xDE, 0xDC, 0xA2, 0x5B,
                         0xD5, 0x41, 0x13, 0x9E, 0x6E, 0x05, 0x6D, 0x0A, 0x6C, 0x00, 0x68, 0x33,
                         0x67, 0x0A, 0x66, 0x00, 0x69, 0x00, 0x63, 0x00, 0x00, 0xE0, 0xA2, 0x02,
                         0x60, 0x1C, 0x61, 0x09, 0xD0, 0x1F, 0xA2, 0x59, 0x60, 0x00, 0x61, 0x1F,
                         0xD0, 0x12, 0x70, 0x08, 0x30, 0x40, 0x13, 0xD4, 0xA2, 0x11, 0xDE, 0xDC,
                         0xA2, 0x35, 0xD8, 0x7C, 0x22, 0xE8, 0x3F, 0x00, 0x13, 0xAC, 0x23, 0x60,
                         0x3F, 0x00, 0x13, 0x98, 0xA2, 0x5B, 0x39, 0x00, 0x22, 0x7E, 0x49, 0x00,
                         0x22, 0x5C, 0x33, 0x00, 0x22, 0xC2, 0x13, 0xE4]);
    programs.insert("outlaw_framelock",
                    vec![0x13, 0xB4, 0x38, 0x38, 0x3B, 0x1B, 0x1B, 0x1B, 0xDF, 0xDE, 0xD8, 0xD8,
                         0xF8, 0x78, 0x18, 0x1C, 0x0C, 0x00, 0x00, 0x70, 0xF8, 0x70, 0x67, 0x7C,
                         0x60, 0x60, 0x78, 0x28, 0xEC, 0x18, 0x3E, 0x1C, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x66, 0xC3, 0x18, 0x3E, 0x1C, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x24, 0x36, 0x00, 0x00, 0x06, 0x1F, 0x0E, 0xE6, 0x3E,
                         0x06, 0x06, 0x1E, 0x14, 0x37, 0x18, 0x7C, 0x38, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x66, 0xC3, 0x18, 0x7C, 0x38, 0x18, 0x7E, 0x99, 0x99,
                         0x99, 0x5A, 0x3C, 0x24, 0x6C, 0xFF, 0xFF, 0x80, 0x60, 0x06, 0xE0, 0x9E,
                         0x00, 0xEE, 0x69, 0x01, 0x60, 0x05, 0xE0, 0xA1, 0x69, 0x02, 0x60, 0x08,
                         0xE0, 0xA1, 0x69, 0x03, 0x8B, 0xE0, 0x7B, 0x09, 0x8A, 0xD0, 0x7A, 0x05,
                         0xA2, 0x5B, 0xDB, 0xA1, 0x00, 0xEE, 0xDB, 0xA1, 0x7B, 0x01, 0x49, 0x02,
                         0x7A, 0xFF, 0x49, 0x03, 0x7A, 0x01, 0x4A, 0x01, 0x69, 0x03, 0x4A, 0x1E,
                         0x69, 0x02, 0x4B, 0x3F, 0x69, 0x00, 0x49, 0x00, 0x00, 0xEE, 0xDB, 0xA1,
                         0x4F, 0x00, 0x00, 0xEE, 0x69, 0x00, 0x00, 0xEE, 0x33, 0x00, 0x00, 0xEE,
                         0x63, 0x01, 0xC0, 0x03, 0x40, 0x01, 0x63, 0x02, 0x40, 0x02, 0x63, 0x03,
                         0x85, 0x80, 0x75, 0xFF, 0x84, 0x70, 0x74, 0x05, 0xA2, 0x5B, 0xD5, 0x41,
                         0x00, 0xEE, 0xD5, 0x41, 0x75, 0xFF, 0x43, 0x02, 0x74, 0xFF, 0x43, 0x03,
                         0x74, 0x01, 0x44, 0x01, 0x63, 0x03, 0x44, 0x1E, 0x63, 0x02, 0x45, 0x00,
                         0x63, 0x00, 0x43, 0x00, 0x00, 0xEE, 0xD5, 0x41, 0x4F, 0x00, 0x00, 0xEE,
                         0x63, 0x00, 0x00, 0xEE, 0x81, 0xE0, 0x82, 0xD0, 0x60, 0x07, 0xE0, 0xA1,
                         0x71, 0xFF, 0x60, 0x09, 0xE0, 0xA1, 0x71, 0x01, 0x60, 0x05, 0xE0, 0xA1,
                         0x72, 0xFF, 0x60, 0x08, 0xE0, 0xA1, 0x72, 0x01, 0x60, 0x00, 0x51, 0xE0,
                         0x60, 0x18, 0x52, 0xD0, 0x60, 0x18, 0x4C, 0x18, 0x60, 0x0C, 0x6F, 0x06,
                         0xEF, 0xA1, 0x60, 0x00, 0x41, 0x00, 0x61, 0x01, 0x41, 0x15, 0x61, 0x14,
                         0x42, 0x00, 0x62, 0x01, 0x42, 0x12, 0x62, 0x11, 0xA2, 0x11, 0xFC, 0x1E,
                         0xDE, 0xDC, 0x8E, 0x10, 0x8D, 0x20, 0x8C, 0x00, 0xA2, 0x11, 0xFC, 0x1E,
                         0xDE, 0xDC, 0x00, 0xEE, 0x71, 0xFF, 0x00, 0xEE, 0x71, 0x01, 0x00, 0xEE,
                         0x72, 0xFF, 0x00, 0xEE, 0x72, 0xFF, 0x00, 0xEE, 0x72, 0x01, 0x00, 0xEE,
                         0x72, 0x01, 0x00, 0xEE, 0x80, 0x00, 0x00, 0xEE, 0x22, 0xA4, 0x00, 0xEE,
                         0xC0, 0x1C, 0xB3, 0x3C, 0x81, 0x80, 0x82, 0x70, 0x23, 0x5C, 0x60, 0x00,
                         0x51, 0x80, 0x60, 0x18, 0x52, 0x70, 0x60, 0x18, 0x46, 0x18, 0x60, 0x0C,
                         0x41, 0x23, 0x61, 0x24, 0x41, 0x38, 0x61, 0x37, 0x42, 0x00, 0x62, 0x01,
                         0x42, 0x12, 0x62, 0x11, 0xA2, 0x35, 0xF6, 0x1E, 0xD8, 0x7C, 0x88, 0x10,
                         0x87, 0x20, 0x86, 0x00, 0xA2, 0x35, 0xF6, 0x1E, 0xD8, 0x7C, 0x00, 0xEE,
                         0xD8, 0x7C, 0xA2, 0x5B, 0xDB, 0xA1, 0x60, 0x20, 0xF0, 0x18, 0xF0, 0x15,
                         0xF0, 0x07, 0x30, 0x00, 0x13, 0xA4, 0x13, 0xB4, 0xDE, 0xDC, 0xA2, 0x5B,
                         0xD5, 0x41, 0x13, 0x9E, 0x6E, 0x05, 0x6D, 0x0A, 0x6C, 0x00, 0x68, 0x33,
                         0x67, 0x0A, 0x66, 0x00, 0x69, 0x00, 0x63, 0x00, 0x00, 0xE0, 0xA2, 0x02,
                         0x60, 0x1C, 0x61, 0x09, 0xD0, 0x1F, 0xA2, 0x59, 0x60, 0x00, 0x61, 0x1F,
                         0xD0, 0x12, 0x70, 0x08, 0x30, 0x40, 0x13, 0xD4, 0xA2, 0x11, 0xDE, 0xDC,
                         0xA2, 0x35, 0xD8, 0x7C, 0x22, 0xE8, 0x3F, 0x00, 0x13, 0xAC, 0x23, 0x60,
                         0x3F, 0x00, 0x13, 0x98, 0xA2, 0x5B, 0x39, 0x00, 0x22, 0x7E, 0x49, 0x00,
                         0x22, 0x5C, 0x33, 0x00, 0x22, 0xC2, 0xFF, 0x07, 0x3F, 0x00, 0x13, 0xFE,
                         0x6F, 0x02, 0xFF, 0x15, 0x13, 0xE4]);
    programs.insert("unpack",
                    vec![0x12, 0xC4, 0x20, 0x20, 0x20, 0x00, 0x20, 0xF0, 0x10, 0x70, 0x00, 0x40,
                         0xE0, 0x90, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0x80,
                         0x80, 0xD0, 0xB0, 0xB0, 0x90, 0x90, 0x90, 0x90, 0x60, 0x60, 0x90, 0x90,
                         0x90, 0x60, 0x90, 0xD0, 0xB0, 0x90, 0x90, 0x90, 0x50, 0x20, 0x40, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x60, 0x90, 0xF0, 0x90, 0x90, 0xF0, 0x90, 0x90,
                         0xF0, 0x80, 0xE0, 0x80, 0x80, 0x80, 0x80, 0xF0, 0x80, 0xE0, 0x80, 0xF0,
                         0x40, 0x40, 0x40, 0xF0, 0x20, 0x40, 0x80, 0xF0, 0x20, 0x20, 0x20, 0xC0,
                         0x60, 0x90, 0x80, 0x90, 0x60, 0x90, 0x90, 0xB0, 0x70, 0x80, 0xB0, 0x90,
                         0x70, 0x80, 0x60, 0x10, 0xE0, 0x90, 0xA0, 0xC0, 0xA0, 0x90, 0x90, 0xB0,
                         0xB0, 0xD0, 0xF0, 0x40, 0x40, 0x40, 0x40, 0x90, 0x90, 0x70, 0x10, 0xE0,
                         0xA2, 0x82, 0xF1, 0x55, 0x61, 0x02, 0x62, 0x01, 0x63, 0x00, 0x00, 0x00,
                         0xF3, 0x1E, 0xF0, 0x65, 0xA2, 0x02, 0xF0, 0x1E, 0xD1, 0x25, 0x71, 0x05,
                         0x41, 0x3E, 0x72, 0x06, 0x41, 0x3E, 0x61, 0x02, 0x73, 0x01, 0x53, 0x40,
                         0x12, 0x82, 0x00, 0xEE, 0x71, 0x1F, 0x1A, 0x2D, 0x35, 0x41, 0x32, 0x0A,
                         0x2D, 0x2D, 0x2D, 0x2D, 0x32, 0x2D, 0x0A, 0x1A, 0x17, 0x10, 0x3D, 0x41,
                         0x2D, 0x45, 0x24, 0x2D, 0x6C, 0x35, 0x41, 0x2D, 0x0C, 0x45, 0x5E, 0x6C,
                         0x32, 0x24, 0x52, 0x41, 0x60, 0xA2, 0x61, 0xA0, 0x64, 0x24, 0x22, 0x78,
                         0x12, 0xCC]);

    programs.insert("bigfont",
                    vec![0x00, 0xFF, 0xF0, 0x30, 0xD1, 0x2A, 0x71, 0x09, 0x70, 0x01, 0x40, 0x08,
                         0x61, 0x00, 0x40, 0x08, 0x72, 0x0B, 0x30, 0x10, 0x12, 0x02, 0x12, 0x16]);

    programs.insert("bench",
                    vec![0x12, 0x7E, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x18, 0x3C, 0x3C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x24, 0x24, 0x18,
                         0x00, 0x00, 0x00, 0x18, 0x24, 0x42, 0x42, 0x24, 0x18, 0x00, 0x18, 0x66,
                         0x42, 0x81, 0x81, 0x42, 0x66, 0x18, 0x18, 0x66, 0x42, 0x99, 0x99, 0x42,
                         0x66, 0x18, 0x18, 0x66, 0x5A, 0xBD, 0xBD, 0x5A, 0x66, 0x18, 0x3C, 0x42,
                         0x99, 0xA5, 0xA5, 0x99, 0x42, 0x3C, 0x0E, 0x8A, 0xAA, 0xCA, 0xAE, 0xEE,
                         0xA8, 0xEE, 0x82, 0x8E, 0x40, 0x40, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6D, 0x00, 0x00, 0xE0, 0x23, 0xB4,
                         0x84, 0xD0, 0x65, 0x80, 0x84, 0x52, 0x34, 0x80, 0x12, 0x82, 0x23, 0x62,
                         0x00, 0xE0, 0x22, 0xA2, 0x12, 0x82, 0x6F, 0x3C, 0xFF, 0x15, 0xFF, 0x07,
                         0x3F, 0x00, 0x12, 0x9A, 0x00, 0xEE, 0x63, 0x00, 0x65, 0x01, 0xA2, 0x7A,
                         0xF0, 0x65, 0xA2, 0x7B, 0xF0, 0x33, 0xA2, 0x7C, 0xF0, 0x65, 0x30, 0x00,
                         0x63, 0x01, 0x43, 0x00, 0x12, 0xC0, 0xF0, 0x29, 0x64, 0x01, 0xD4, 0x55,
                         0xA2, 0x7D, 0xF0, 0x65, 0x30, 0x00, 0x63, 0x01, 0x43, 0x00, 0x12, 0xD2,
                         0xF0, 0x29, 0x64, 0x06, 0xD4, 0x55, 0xA2, 0x79, 0xF0, 0x65, 0xA2, 0x7B,
                         0xF0, 0x33, 0xA2, 0x7C, 0xF0, 0x65, 0x30, 0x00, 0x63, 0x01, 0x43, 0x00,
                         0x12, 0xEC, 0xF0, 0x29, 0x64, 0x0B, 0xD4, 0x55, 0xA2, 0x7D, 0xF0, 0x65,
                         0x30, 0x00, 0x63, 0x01, 0x43, 0x00, 0x13, 0x08, 0xF0, 0x29, 0x64, 0x12,
                         0xD4, 0x55, 0xA2, 0x4C, 0x64, 0x0F, 0x65, 0x05, 0xD4, 0x52, 0x65, 0x01,
                         0xA2, 0x78, 0xF0, 0x65, 0xA2, 0x7B, 0xF0, 0x33, 0xA2, 0x7C, 0xF0, 0x65,
                         0x30, 0x00, 0x63, 0x01, 0x43, 0x00, 0x13, 0x22, 0xF0, 0x29, 0x64, 0x17,
                         0xD4, 0x55, 0xA2, 0x7D, 0xF0, 0x65, 0xF0, 0x29, 0x64, 0x1C, 0xD4, 0x55,
                         0xA2, 0x4E, 0x64, 0x21, 0x65, 0x05, 0xD4, 0x51, 0x65, 0x01, 0xA2, 0x77,
                         0xF0, 0x65, 0xA2, 0x7B, 0xF0, 0x33, 0xA2, 0x7C, 0xF0, 0x65, 0xF0, 0x29,
                         0x64, 0x23, 0xD4, 0x55, 0xA2, 0x7D, 0xF0, 0x65, 0xF0, 0x29, 0x64, 0x28,
                         0xD4, 0x55, 0x64, 0x2E, 0x65, 0x01, 0xA2, 0x42, 0xD4, 0x55, 0x74, 0x08,
                         0xA2, 0x47, 0xD4, 0x55, 0x00, 0xEE, 0x63, 0x00, 0x64, 0x00, 0x65, 0x00,
                         0x66, 0x00, 0x67, 0x00, 0x69, 0x64, 0xA2, 0x4F, 0xF2, 0x65, 0x83, 0x04,
                         0x88, 0x30, 0x88, 0x95, 0x4F, 0x00, 0x13, 0x80, 0x83, 0x80, 0x74, 0x01,
                         0x84, 0x14, 0x88, 0x40, 0x88, 0x95, 0x4F, 0x00, 0x13, 0x8E, 0x84, 0x80,
                         0x75, 0x01, 0x85, 0x24, 0x88, 0x50, 0x88, 0x95, 0x4F, 0x00, 0x13, 0x9C,
                         0x85, 0x80, 0x76, 0x01, 0x68, 0x01, 0xF8, 0x1E, 0x77, 0x01, 0x37, 0x0A,
                         0x13, 0x70, 0xA2, 0x77, 0x80, 0x30, 0x81, 0x40, 0x82, 0x50, 0x83, 0x60,
                         0xF3, 0x55, 0x00, 0xEE, 0x60, 0x00, 0x61, 0x00, 0x62, 0x00, 0x6F, 0x20,
                         0xFF, 0x15, 0xFF, 0x07, 0x3F, 0x1E, 0x13, 0xBE, 0x23, 0xFE, 0x70, 0x01,
                         0x40, 0x64, 0x71, 0x01, 0x30, 0x64, 0x70, 0x64, 0x70, 0x9C, 0x41, 0x64,
                         0x72, 0x01, 0x31, 0x64, 0x71, 0x64, 0x71, 0x9C, 0xFF, 0x07, 0x3F, 0x00,
                         0x13, 0xC4, 0xA2, 0x4F, 0x85, 0xD0, 0x66, 0x7F, 0x85, 0x62, 0x7D, 0x01,
                         0x4D, 0x0A, 0x6D, 0x80, 0x4D, 0x8A, 0x6D, 0x80, 0x85, 0x5E, 0x85, 0x5E,
                         0xF5, 0x1E, 0xF2, 0x55, 0x00, 0xEE, 0x24, 0x16, 0x24, 0x0E, 0x24, 0x1C,
                         0x24, 0x16, 0x24, 0x26, 0x24, 0x0E, 0x24, 0x1C, 0x00, 0xEE, 0xA2, 0x02,
                         0xC6, 0x38, 0xF6, 0x1E, 0x00, 0xEE, 0xC6, 0x0F, 0xF6, 0x29, 0x00, 0xEE,
                         0xC3, 0x3F, 0xC4, 0x0F, 0x74, 0x08, 0xD3, 0x48, 0x00, 0xEE, 0xC3, 0x3F,
                         0xC4, 0x0F, 0x74, 0x08, 0xD3, 0x45, 0x00, 0xEE]);

    programs
}
