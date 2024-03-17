const std = @import("std");
const sol = @import("sol/sol.zig");

pub fn build(b: *std.build.Builder) !void {
    const optimize = .ReleaseSmall;
    const program = b.addSharedLibrary(.{
        .name = "helloworld",
        .root_source_file = .{ .path = "src/main.zig" },
        .optimize = optimize,
        .target = sol.sbf_target,
    });
    try sol.buildProgram(b, program, "sol/");

    const test_step = b.step("test", "Run unit tests");
    const unit_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
    });
    sol.addSolModules(b, unit_tests, "sol/");
    const run_unit_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
