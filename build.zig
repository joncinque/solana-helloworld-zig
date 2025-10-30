const std = @import("std");
const solana = @import("solana_program_sdk");
const base58 = @import("base58");

pub fn build(b: *std.Build) !void {
    // Be sure to specify a solana target
    const sbf_target = b.resolveTargetQuery(solana.sbf_target);
    const optimize = .ReleaseFast;

    const mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .optimize = optimize,
        .target = sbf_target,
    });

    const program = b.addLibrary(.{
        .name = "helloworld",
        .linkage = .dynamic,
        .root_module = mod,
    });

    // Adding required dependencies, link the program properly, and get a
    // prepared solana-program module
    const solana_mod = solana.buildProgram(b, program, sbf_target, optimize);

    // Install the program artifact
    b.installArtifact(program);

    // Optional: generate a keypair for the program
    base58.generateProgramKeypair(b, program);

    // Run unit tests
    const target = b.standardTargetOptions(.{});
    const test_mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .optimize = optimize,
        .target = target,
    });
    const test_step = b.step("test", "Run unit tests");
    const lib_unit_tests = b.addTest(.{
        .root_module = test_mod,
    });
    lib_unit_tests.root_module.addImport("solana_program_sdk", solana_mod);
    const run_unit_tests = b.addRunArtifact(lib_unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
