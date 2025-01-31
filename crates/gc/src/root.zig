const std = @import("std");
const alloc = @import("alloc.zig");
const Header = alloc.Header;
const pointerSize = @sizeOf(usize);

var worklist = std.ArrayList(*Header).init(alloc.allocator);
var roots = std.ArrayList([*]u8).init(alloc.allocator);

// maskOrPointer gets truncated to the lower 48 bits
export fn allocLayout(size: usize, typ: alloc.ObjType, maskLen: u16, maskOrPointer: u64) ?[*]u8 {
    const header = alloc.alloc(size) catch return null;
    header.typ = typ;
    header.maskLen = maskLen;
    header.maskOrPointer = @truncate(maskOrPointer);
    return header.data().ptr;
}

export fn allocLeaf(size: usize) ?[*]u8 {
    return allocLayout(size, .leaf, 0, 0);
}

export fn allocSliceOfPointers(len: usize) ?[*][*]u8 {
    return @ptrCast(@alignCast(allocLayout(len * @sizeOf([*]u8), .slice_of_pointers, 0, 0)));
}

export fn allocSliceOfFatPointers(len: usize) ?[*][]u8 {
    return @ptrCast(@alignCast(allocLayout(len * @sizeOf([]u8), .slice_of_fat_pointers, 0, 0)));
}

export fn addRoot(pointer: [*]u8) void {
    roots.append(pointer) catch @panic("Memory allocation error!");
}

export fn removeRoot(pointer: [*]u8) void {
    for (roots.items, 0..) |root, i| {
        if (root == pointer) {
            _ = roots.swapRemove(i);
            return;
        }
    }
}

export fn collectGarbage() void {
    markRoots();
    markRest();
    sweep();
}

fn markRoots() void {
    for (roots.items) |root| {
        markObject(followDataPointer(root) orelse continue);
    }
}

fn markRest() void {
    while (worklist.popOrNull()) |header| {
        markObject(header);
    }
}

fn markObject(header: *Header) void {
    if (header.marked) {
        return;
    }
    header.marked = true;

    switch (header.typ) {
        .leaf => return,
        .small_mask => return,
        .large_mask => return,
        .slice_of_small_mask => return,
        .slice_of_large_mask => return,
        .slice_of_pointers => {
            const len = header.len / @sizeOf([*]u8);
            const slice = @as([*][*]u8, @ptrCast(@alignCast(header.data())))[0..len];

            for (slice) |pointer| {
                if (followDataPointer(pointer)) |child| {
                    worklist.append(child) catch @panic("Allocation error!");
                }
            }
        },
        .slice_of_fat_pointers => {
            const len = header.len / @sizeOf([]u8);
            const slice = @as([*][]u8, @ptrCast(@alignCast(header.data())))[0..len];

            for (slice) |pointer| {
                if (followDataPointer(pointer.ptr)) |child| {
                    worklist.append(child) catch @panic("Allocation error!");
                }
            }
        },
    }
}

fn sweep() void {
    var prev: ?*Header = null;
    var maybeNext = alloc.objects.next;
    var i: usize = 0;
    while (maybeNext) |next| {
        i += 1;
        const nextNext = next.next;

        if (next.marked) {
            next.marked = false;
            prev = next;
        } else {
            if (prev) |actualPrev| {
                actualPrev.next = next.next;
            } else {
                // First node deleted
                alloc.objects.next = next.next;
            }
            alloc.free(next);
        }
        maybeNext = nextNext;
    }
}

// Gets the corresponding header of a pointer pointing into the data section of an object
fn followDataPointer(pointer: [*]u8) ?*Header {
    const pointerInt = @intFromPtr(pointer);
    var maybeNext = alloc.objects.next;

    while (maybeNext) |next| {
        const data = next.data();
        const dataStart = @intFromPtr(data.ptr);
        const dataEnd = dataStart + data.len;
        if ((dataStart <= pointerInt) and (pointerInt <= dataEnd)) {
            return next;
        }
        maybeNext = next.next;
    }

    return null;
}

test "alloc and collect slice of pointers" {
    const leaf0 = allocLeaf(4) orelse return;
    const leaf1 = allocLeaf(16) orelse return;

    const leaf1Slice = allocSliceOfFatPointers(1) orelse return;
    leaf1Slice[0] = leaf1[0..8];

    const slice = allocSliceOfPointers(2) orelse return;
    slice[0] = leaf0;
    slice[1] = @ptrCast(leaf1Slice);

    addRoot(@ptrCast(slice));
    std.debug.print("{}", .{alloc.objects});

    collectGarbage();
    std.debug.print("{}", .{alloc.objects});
    try std.testing.expectEqual(1, roots.items.len);
    try std.testing.expectEqual(4, alloc.objects.len());

    slice[1] = leaf1;

    collectGarbage();
    std.debug.print("{}", .{alloc.objects});
    try std.testing.expectEqual(1, roots.items.len);
    try std.testing.expectEqual(3, alloc.objects.len());

    removeRoot(@ptrCast(slice));

    collectGarbage();
    std.debug.print("{}", .{alloc.objects});
    try std.testing.expectEqual(0, roots.items.len);
    try std.testing.expectEqual(0, alloc.objects.len());
}
