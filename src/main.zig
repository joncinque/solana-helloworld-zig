const std = @import("std");
const sol = @import("solana-program-sdk");

const ix = @import("instruction.zig");
const state = @import("state.zig");
const ProgramError = @import("error.zig").ProgramError;

export fn entrypoint(input: [*]u8) u64 {
    var context = sol.Context.load(input) catch return 1;
    processInstruction(context.program_id, context.accounts[0..context.num_accounts], context.data) catch |err| return @intFromError(err);
    return 0;
}

fn processInstruction(program_id: *sol.PublicKey, accounts: []sol.Account, data: []const u8) ProgramError!void {
    sol.print("Hello zig program {s}", .{program_id});
    for (accounts, 0..) |account, i| {
        sol.print("Hello Account {}: {s}", .{ i, account.info().owner_id });
    }
    sol.print("Hello Data: {any}", .{data});
    const instruction_type: *const ix.InstructionType = @ptrCast(data);
    switch (instruction_type.*) {
        ix.InstructionType.init => {
            const init: *align(1) const ix.InitData = @ptrCast(data[1..]);
            sol.print("Hello init {}", .{init.account_type});
            const account = accounts[0];
            const account_type: state.AccountType = @enumFromInt(account.data()[0]);
            if (account_type != state.AccountType.Uninitialized) {
                return ProgramError.AlreadyInUse;
            }
            if (init.account_type == state.AccountType.Uninitialized) {
                return ProgramError.InvalidAccountType;
            }
            if (account.data()[1..].len != init.account_type.sizeOfData()) {
                return ProgramError.IncorrectSize;
            }
            account.data()[0] = @intFromEnum(init.account_type);
        },
        ix.InstructionType.increment => {
            sol.log("Hello increment");
            const account = accounts[0];
            const account_type: state.AccountType = @enumFromInt(account.data()[0]);
            switch (account_type) {
                state.AccountType.Uninitialized => return ProgramError.Uninitialized,
                state.AccountType.SmallInt => {
                    var accountData: *align(1) state.SmallIntData = @ptrCast(account.data()[1..]);
                    accountData.amount += 1;
                },
                state.AccountType.BigInt => {
                    var accountData: *align(1) state.BigIntData = @ptrCast(account.data()[1..]);
                    accountData.amount += 1;
                },
            }
        },
        ix.InstructionType.add => {
            const add: *align(1) const ix.AddData = @ptrCast(data[1..]);
            sol.print("Hello add {}", .{add.amount});
            const account = accounts[0];
            const account_type: state.AccountType = @enumFromInt(account.data()[0]);
            switch (account_type) {
                state.AccountType.Uninitialized => return ProgramError.Uninitialized,
                state.AccountType.SmallInt => {
                    var accountData: *align(1) state.SmallIntData = @ptrCast(account.data()[1..]);
                    accountData.amount += add.amount;
                },
                state.AccountType.BigInt => {
                    var accountData: *align(1) state.BigIntData = @ptrCast(account.data()[1..]);
                    accountData.amount += add.amount;
                },
            }
        },
    }
}

test {
    std.testing.refAllDecls(@This());
}
