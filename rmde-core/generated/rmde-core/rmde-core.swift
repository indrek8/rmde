
public class RMDEEditor: RMDEEditorRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$RMDEEditor$_free(ptr)
        }
    }
}
extension RMDEEditor {
    public convenience init() {
        self.init(ptr: __swift_bridge__$RMDEEditor$new())
    }
}
public class RMDEEditorRefMut: RMDEEditorRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
extension RMDEEditorRefMut {
    public func new_tab() -> UInt64 {
        __swift_bridge__$RMDEEditor$new_tab(ptr)
    }

    public func close_tab(_ id: UInt64) -> Bool {
        __swift_bridge__$RMDEEditor$close_tab(ptr, id)
    }

    public func switch_tab(_ id: UInt64) -> Bool {
        __swift_bridge__$RMDEEditor$switch_tab(ptr, id)
    }

    public func next_tab() {
        __swift_bridge__$RMDEEditor$next_tab(ptr)
    }

    public func prev_tab() {
        __swift_bridge__$RMDEEditor$prev_tab(ptr)
    }

    public func insert_text<GenericToRustStr: ToRustStr>(_ text: GenericToRustStr) {
        text.toRustStr({ textAsRustStr in
            __swift_bridge__$RMDEEditor$insert_text(ptr, textAsRustStr)
        })
    }

    public func delete_backward() {
        __swift_bridge__$RMDEEditor$delete_backward(ptr)
    }

    public func delete_forward() {
        __swift_bridge__$RMDEEditor$delete_forward(ptr)
    }

    public func set_cursor(_ pos: UInt) {
        __swift_bridge__$RMDEEditor$set_cursor(ptr, pos)
    }

    public func add_cursor(_ pos: UInt) {
        __swift_bridge__$RMDEEditor$add_cursor(ptr, pos)
    }

    public func move_cursors(_ delta: Int64, _ extend: Bool) {
        __swift_bridge__$RMDEEditor$move_cursors(ptr, delta, extend)
    }

    public func select_all() {
        __swift_bridge__$RMDEEditor$select_all(ptr)
    }

    public func open_file<GenericToRustStr: ToRustStr>(_ path: GenericToRustStr) -> RustString {
        return path.toRustStr({ pathAsRustStr in
            RustString(ptr: __swift_bridge__$RMDEEditor$open_file(ptr, pathAsRustStr))
        })
    }

    public func save_file() -> RustString {
        RustString(ptr: __swift_bridge__$RMDEEditor$save_file(ptr))
    }

    public func save_file_as<GenericToRustStr: ToRustStr>(_ path: GenericToRustStr) -> RustString {
        return path.toRustStr({ pathAsRustStr in
            RustString(ptr: __swift_bridge__$RMDEEditor$save_file_as(ptr, pathAsRustStr))
        })
    }
}
public class RMDEEditorRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension RMDEEditorRef {
    public func tab_count() -> UInt {
        __swift_bridge__$RMDEEditor$tab_count(ptr)
    }

    public func get_active_tab_id() -> UInt64 {
        __swift_bridge__$RMDEEditor$get_active_tab_id(ptr)
    }

    public func get_content() -> RustString {
        RustString(ptr: __swift_bridge__$RMDEEditor$get_content(ptr))
    }

    public func get_content_length() -> UInt {
        __swift_bridge__$RMDEEditor$get_content_length(ptr)
    }

    public func get_cursor_position() -> UInt {
        __swift_bridge__$RMDEEditor$get_cursor_position(ptr)
    }

    public func is_dirty() -> Bool {
        __swift_bridge__$RMDEEditor$is_dirty(ptr)
    }

    public func get_title() -> RustString {
        RustString(ptr: __swift_bridge__$RMDEEditor$get_title(ptr))
    }
}
extension RMDEEditor: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_RMDEEditor$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_RMDEEditor$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: RMDEEditor) {
        __swift_bridge__$Vec_RMDEEditor$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_RMDEEditor$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (RMDEEditor(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RMDEEditorRef> {
        let pointer = __swift_bridge__$Vec_RMDEEditor$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RMDEEditorRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<RMDEEditorRefMut> {
        let pointer = __swift_bridge__$Vec_RMDEEditor$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return RMDEEditorRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<RMDEEditorRef> {
        UnsafePointer<RMDEEditorRef>(OpaquePointer(__swift_bridge__$Vec_RMDEEditor$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_RMDEEditor$len(vecPtr)
    }
}



