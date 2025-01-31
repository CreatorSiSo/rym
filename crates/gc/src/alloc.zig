const std = @import("std");
const AllocError = std.mem.Allocator.Error;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
pub var roots = std.ArrayList([*]u8).init(allocator);

const Objects = struct {
    next: ?*Header,
    len: usize,

    fn init() Objects {
        return .{ .next = null, .len = 0 };
    }

    pub fn push(self: *Objects, object: *Header) void {
        object.next = self.next;
        self.next = object;
    }

    pub fn format(self: *const Objects, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        var maybeNext = self.next;
        try writer.writeAll("Objects [\n");

        while (maybeNext) |next| {
            try writer.writeAll("  ");
            try std.fmt.formatType(next, "?", .{}, writer, 1);
            try writer.writeAll(",\n");
            maybeNext = next.next;
        }

        return writer.writeAll("]\n");
    }
};
pub var objects = Objects.init();

const Allocation = []align(@alignOf(Header)) u8;

pub const ObjType = enum(u8) {
    // Does not contain any pointers
    leaf = 0,
    // Masks the locations of a maximum of 64 pointers
    small_mask = 1,
    // Masks more than 64 locations of pointers
    large_mask = 2,
    slice_of_small_mask = 3,
    slice_of_large_mask = 4,
    // Contains pointers into data segments
    slice_of_pointers = 5,
    // Contains fat pointers into data segments
    slice_of_fat_pointers = 6,
};

pub const Header = struct {
    next: ?*Header,
    len: usize,
    marked: bool,
    typ: ObjType,
    maskOrPointer: usize,

    pub fn fromAllocation(allocation: Allocation) *Header {
        return std.mem.bytesAsValue(Header, allocation);
    }

    pub fn toAllocation(self: *Header) Allocation {
        const size = @sizeOf(Header) + self.len;
        return @alignCast(@as([*]u8, @ptrCast(self))[0..size]);
    }

    pub fn data(self: *Header) []u8 {
        const start = @sizeOf(Header);
        const end = start + self.len;
        return @as([*]u8, @ptrCast(self))[start..end];
    }
};

pub fn alloc(size: usize) AllocError!*Header {
    const allocation = try allocator.alignedAlloc(u8, @alignOf(Header), @sizeOf(Header) + size);
    const header = Header.fromAllocation(allocation);
    header.* = std.mem.zeroInit(Header, .{ .len = size });
    objects.push(header);
    return header;
}

pub fn free(header: *Header) void {
    allocator.free(header.toAllocation());
}

test "Header details" {
    std.debug.print("size: {}, align: {}\n", .{ @sizeOf(Header), @alignOf(Header) });
    try std.testing.expectEqual(@sizeOf(Header), @sizeOf(*void) * 4);
    try std.testing.expectEqual(@alignOf(Header), @alignOf(*void));
}
