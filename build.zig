const std = @import("std");
const sol = @import("sol/sol.zig");

pub fn build(b: *std.build.Builder) !void {
    const optimize = .ReleaseSmall;
    const program = b.addSharedLibrary(.{
        .name = "helloworld",
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = sol.bpf_target,
    });
    try sol.buildProgram(b, program, "sol/");
}
