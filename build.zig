const std = @import("std");
const sol = @import("sol/sol.zig");

pub fn build(b: *std.build.Builder) !void {
    const optimize = .ReleaseSmall;
    const program = b.addSharedLibrary(.{
        .name = "helloworld",
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = sol.program.bpf_target,
    });
    try sol.program.buildProgram(b, program, "sol/");
}
