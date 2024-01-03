const std = @import("std");
const sol = @import("sol");

export fn entrypoint(input: [*]u8) u64 {
    const context = sol.Context.load(input) catch return 1;
    const accounts = context.loadRawAccounts(sol.allocator) catch return 1;
    defer accounts.deinit();
    processInstruction(context.program_id, accounts.items, context.data) catch return 1;
    return 0;
}

fn processInstruction(program_id: *sol.PublicKey, accounts: []sol.Account, data: []const u8) !void {
    sol.print("Hello zig program {s}", .{program_id});
    for (accounts, 0..) |account, i| {
        sol.print("Hello Account {}: {s}", .{i, account.info().owner_id});
    }
    sol.print("Hello Data: {s}", .{data});
}
