const std = @import("std");
const AllocError = std.mem.Allocator.Error;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
pub var all_metadata = std.ArrayList(Metadata).init(allocator);

const Allocation = []align(@alignOf(Header)) u8;

pub const Metadata = struct {
    header: *Header,
    data: []u8,
    allocation: Allocation,
};

fn to_metadata(allocation: Allocation) Metadata {
    return .{
        .header = @ptrCast(allocation[0..@sizeOf(Header)]),
        .data = allocation[@sizeOf(Header)..],
        .allocation = allocation,
    };
}

pub const Header = struct {
    typ: enum(u32) {
        leaf = 0,
        slice_of_pointers = 1,
        slice_of_fat_pointers = 2,
        // Custom types (structs, tuples and tagged unions) are possible as well
        _,
    },
    marked: bool,
    len: usize,
};

pub fn alloc(size: usize) AllocError!Metadata {
    const bytes = try allocator.alignedAlloc(u8, @alignOf(Header), @sizeOf(Header) + size);
    const metadata = to_metadata(bytes);

    metadata.header.typ = .leaf;
    metadata.header.marked = false;
    metadata.header.len = size;

    try all_metadata.append(metadata);
    return metadata;
}

pub fn free(metadata: Metadata) void {
    allocator.free(metadata.allocation);
}

test "size of Header" {
    std.debug.print("{}\n", .{@sizeOf(Header)});
}
