const std = @import("std");
const AccountType = @import("state.zig").AccountType;

const testing = std.testing;

pub const InstructionType = enum(u8) {
    init,
    increment,
    add,
};

pub const InitData = packed struct {
    account_type: AccountType,
};

pub const AddData = packed struct {
    amount: u32,
};

test "instruction type cast" {
    const inc = InstructionType.increment;
    const add = InstructionType.add;

    // can work with a ptr, but brittle
    const inc_type = [_]u8{1};
    const inc_ptr: *const InstructionType = @ptrCast(&inc_type);
    try testing.expectEqual(inc, inc_ptr.*);

    // preferred way, directly from the value
    const add_type = [_]u8{ 0, 2 };
    const add_ptr: InstructionType = @enumFromInt(add_type[1]);
    try testing.expectEqual(add, add_ptr);
}

test "instruction data cast" {
    const add_type = [_]u8{ 2, 255, 255, 255, 255 };
    const data_ptr = @intFromPtr(&add_type) + @sizeOf(InstructionType);
    const add_ptr: *align(1) const AddData = @ptrCast(@as([*]u8, @ptrFromInt(data_ptr)));
    try testing.expectEqual(add_ptr.amount, std.math.maxInt(u32));
}
