const std = @import("std");
const solana = @import("solana-program-sdk");

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(solana.sbf_target);
    const optimize = .ReleaseSmall;
    const program = b.addSharedLibrary(.{
        .name = "helloworld",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Adding required dependencies, link the program properly, and get a
    // prepared modules
    const solana_mod = solana.buildProgram(b, program, target, optimize);

    const test_step = b.step("test", "Run unit tests");
    const lib_unit_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
    });
    lib_unit_tests.root_module.addImport("solana-program-sdk", solana_mod);
    const run_unit_tests = b.addRunArtifact(lib_unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
