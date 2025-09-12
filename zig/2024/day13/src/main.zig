const std = @import("std");

const lib = @import("day13_lib");

const Allocator = std.mem.Allocator;

const Grid = struct {
    width: usize,
    height: usize,
    tiles: std.ArrayList(Tile),

    fn print(self: *const Grid) !void {
        var buf: [4]u8 = undefined;
        for (self.tiles.items, 0..) |t, idx| {
            if (idx % self.width == 0) {
                std.debug.print("\n", .{});
            }
            const c: u21 = switch (t) {
                .empty => '.',
                .wall => '#',
                .player => |d| switch (d) {
                    .north => '⮝',
                    .east => '⮞',
                    .south => '⮟',
                    .west => '⮜',
                },
                .travelled => |d| switch (d) {
                    .north => '^',
                    .east => '>',
                    .south => '⌄',
                    .west => '<',
                },
            };
            const written: u3 = try std.unicode.utf8Encode(c, &buf);
            std.debug.print("{s}", .{buf[0..written]});
        }
        std.debug.print("\n", .{});
    }
};

const Tile = union(enum) {
    empty,
    wall,
    player: Dir,
    travelled: Dir,
};

const Dir = enum { north, east, south, west };

const Point = struct {
    x: usize,
    y: usize,

    fn init(x: usize, y: usize) @This() {
        return .{ .x = x, .y = y };
    }
};

const Player = struct {
    pos: Point,
    dir: Dir,

    fn init() @This() {
        return @This(){
            .pos = Point.init(0, 0),
            .dir = Dir.east,
        };
    }
};

fn part1(alloc: Allocator, input: []const u8) !u32 {
    const width: usize = std.mem.indexOfScalar(u8, input, '\n') orelse input.len - 1;
    const height: usize = (input.len + 1) / (width + 1);
    var tiles = std.ArrayList(Tile).init(alloc);
    defer tiles.deinit();

    var end: Point = Point.init(0, 0);
    var player: Player = Player.init();
    try tiles.resize(width * height);

    for (input, 0..) |c, input_idx| {
        if (c == '\n') {
            continue;
        }
        const idx = input_idx - (input_idx / (width + 1)); // Offset for newlines
        const p: Point = .{ .x = idx % width, .y = idx / width };

        tiles.items[idx] = if (c == '#') Tile.wall else Tile.empty;
        if (c == 'S') {
            player.pos = p;
            tiles.items[idx] = Tile{ .player = Dir.east };
        }
        if (c == 'E') {
            end = p;
        }
    }

    const grid: Grid = .{
        .height = height,
        .width = width,
        .tiles = tiles,
    };
    try grid.print();

    return 0;
}

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var gpa = std.heap.DebugAllocator(.{}).init;
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const input = @embedFile("input.txt");

    const result = try part1(alloc, input);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // Don't forget to flush!
}
