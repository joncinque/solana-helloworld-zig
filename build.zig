const std = @import("std");
const sol = @import("sol/build.zig");

const sol_pkgs = sol.Packages("sol/");

pub fn build(b: *std.build.Builder) !void {
    const program = b.addSharedLibrary("helloworld", "src/main.zig", .unversioned);
    inline for (@typeInfo(sol_pkgs).Struct.decls) |field| {
        program.addPackage(@field(sol_pkgs, field.name));
    }
    program.install();

    try sol.linkSolanaProgram(b, program);
    try sol.generateProgramKeypair(b, program);
}
