//! Built-in pet catalog ported from the Codex App avatar catalog.

pub(super) const DEFAULT_FRAME_WIDTH: u32 = 192;
pub(super) const DEFAULT_FRAME_HEIGHT: u32 = 208;
pub(super) const DEFAULT_FRAME_COLUMNS: u32 = 8;
pub(super) const DEFAULT_FRAME_ROWS: u32 = 9;
pub(super) const SPRITESHEET_WIDTH: u32 = DEFAULT_FRAME_WIDTH * DEFAULT_FRAME_COLUMNS;
pub(super) const SPRITESHEET_HEIGHT: u32 = DEFAULT_FRAME_HEIGHT * DEFAULT_FRAME_ROWS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BuiltinPet {
    pub(super) id: &'static str,
    pub(super) display_name: &'static str,
    pub(super) description: &'static str,
    pub(super) spritesheet_file: &'static str,
}

pub(super) const BUILTIN_PETS: &[BuiltinPet] = &[
    BuiltinPet {
        id: "codex",
        display_name: "Codex",
        description: "Người bạn đồng hành Codex gốc",
        spritesheet_file: "codex-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "dewey",
        display_name: "Dewey",
        description: "Một chú vịt gọn gàng cho những ngày làm việc thư thái",
        spritesheet_file: "dewey-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "fireball",
        display_name: "Fireball",
        description: "Năng lượng đường nóng cho vòng lặp nhanh",
        spritesheet_file: "fireball-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "rocky",
        display_name: "Rocky",
        description: "Hòn đá vững chãi khi diff trở nên lớn",
        spritesheet_file: "rocky-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "seedy",
        display_name: "Seedy",
        description: "Mầm xanh nhỏ cho những ý tưởng mới",
        spritesheet_file: "seedy-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "stacky",
        display_name: "Stacky",
        description: "Một chồng cân bằng cho công việc sâu",
        spritesheet_file: "stacky-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "bsod",
        display_name: "BSOD",
        description: "Một con yêu tinh màn hình xanh bé xíu",
        spritesheet_file: "bsod-spritesheet-v4.webp",
    },
    BuiltinPet {
        id: "null-signal",
        display_name: "Null Signal",
        description: "Tín hiệu lặng lẽ từ khoảng không",
        spritesheet_file: "null-signal-spritesheet-v4.webp",
    },
];

pub(super) fn builtin_pet(id: &str) -> Option<BuiltinPet> {
    BUILTIN_PETS.iter().copied().find(|pet| pet.id == id)
}

#[cfg(test)]
pub(super) fn write_test_spritesheet(path: &std::path::Path) {
    let image = image::RgbaImage::new(SPRITESHEET_WIDTH, SPRITESHEET_HEIGHT);
    image.save(path).unwrap();
}
