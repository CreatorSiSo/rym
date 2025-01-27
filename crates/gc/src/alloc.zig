const std = @import("std");
const AllocError = std.mem.Allocator.Error;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
pub var objects = std.ArrayList(*Header).init(allocator);
pub var roots = std.ArrayList([*]u8).init(allocator);

const Allocation = []align(@alignOf(Header)) u8;

const ObjType = enum(u32) {
    // Does not contain any pointers
    leaf = 0,
    // Contains pointers to the start of a data segment
    slice_of_pointers = 1,
    // Contains fat pointers into some data segment
    slice_of_fat_pointers = 2,
    // Custom types (structs, tuples and tagged unions) are possible as well
    _,
};

pub const Header = struct {
    typ: ObjType,
    marked: bool,
    len: usize,

    pub fn fromAllocation(allocation: Allocation) *Header {
        return @ptrCast(allocation[0..@sizeOf(Header)]);
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
    header.typ = .leaf;
    header.marked = false;
    header.len = size;
    try objects.append(header);
    return header;
}

pub fn free(header: *Header) void {
    allocator.free(header.toAllocation());
}

test "Header details" {
    std.debug.print("size: {}, align: {}\n", .{ @sizeOf(Header), @alignOf(Header) });
}
