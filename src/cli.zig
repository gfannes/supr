const std = @import("std");

const cli = @import("rubr").cli;

pub const Error = error{
    CouldNotFindExeName,
};

pub const Options = struct {
    const Self = @This();

    exe: []const u8 = &.{},
    help: bool = false,

    args: cli.Args = undefined,

    pub fn init(ma: std.mem.Allocator) Self {
        return Self{ .args = cli.Args.init(ma) };
    }

    pub fn parse(self: *Self) !void {
        try self.args.setupFromOS();

        self.exe = self.args.pop() orelse return Error.CouldNotFindExeName;
    }

    pub fn deinit(self: *Self) void {
        self.args.deinit();
    }

    pub fn print_help(self: Self, out: std.fs.File.Writer) void {
        try out.print("Help for {s}\n", .{self.exe});
    }
};
