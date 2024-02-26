pub const AccountType = enum(u8) {
    Uninitialized,
    SmallInt,
    BigInt,

    pub fn sizeOfData(self: AccountType) usize {
        return switch (self) {
            AccountType.Uninitialized => 0,
            AccountType.SmallInt => @sizeOf(SmallIntData),
            AccountType.BigInt => @sizeOf(BigIntData),
        };
    }
};

pub const SmallIntData = packed struct { amount: u32 };

pub const BigIntData = packed struct { amount: u256 };
