use crate::{
    Operation,
    ctx::{Ctx, OperationId, Ptr},
};

pub trait LinkedNode {
    fn ahead(&self) -> Option<Ptr>;
    fn behind(&self) -> Option<Ptr>;
    fn ahead_mut(&mut self) -> &mut Option<Ptr>;
    fn behind_mut(&mut self) -> &mut Option<Ptr>;
}

pub struct LinkedListIter<'a> {
    ctx: &'a Ctx,
    current: Option<Ptr>,
}

impl<'a> Iterator for LinkedListIter<'a> {
    type Item = Ptr;
    fn next(&mut self) -> Option<Self::Item> {
        let curr_ptr = self.current?;
        let node = self.ctx.get_operation(OperationId(curr_ptr.idx));
        self.current = node.ahead();
        Some(curr_ptr)
    }
}

pub trait LinkedList {
    fn head(&self) -> &Option<Ptr>;
    fn tail(&self) -> &Option<Ptr>;
    fn head_mut(&mut self) -> &mut Option<Ptr>;
    fn tail_mut(&mut self) -> &mut Option<Ptr>;

    fn insert_behind(&mut self, ctx: &mut Ctx, root: Ptr, inserted: Operation) -> Ptr {
        let op_id = ctx.alloc_operation(inserted);
        let inserted_ptr = Ptr::new(op_id.0);

        if let Some(behind) = ctx.get_operation_mut(OperationId(root.idx)).behind() {
            // link up inserted node between the old behind node and the root
            *ctx.get_operation_mut(OperationId(inserted_ptr.idx))
                .behind_mut() = Some(behind);
            *ctx.get_operation_mut(OperationId(inserted_ptr.idx))
                .ahead_mut() = Some(root);
            
            // the old behind node now points to inserted
            *ctx.get_operation_mut(OperationId(behind.idx)).ahead_mut() = Some(inserted_ptr);

            // point the root's behind ptr to the inserted node
            *ctx.get_operation_mut(OperationId(root.idx)).behind_mut() = Some(inserted_ptr);
        }
        inserted_ptr
    }

    fn push(&mut self, ctx: &mut Ctx, node: crate::ir::Operation) -> Ptr {
        let op_id = ctx.alloc_operation(node);
        let node_ptr = Ptr::new(op_id.0);

        if let Some(tail_ptr) = *self.tail_mut() {
            let tail = ctx.get_operation_mut(OperationId(tail_ptr.idx));
            *tail.ahead_mut() = Some(node_ptr);
            let node = ctx.get_operation_mut(OperationId(node_ptr.idx));
            *node.behind_mut() = Some(tail_ptr)
        }
        *self.tail_mut() = Some(node_ptr);
        if self.head().is_none() {
            *self.head_mut() = Some(node_ptr);
        }
        node_ptr
    }

    fn iter<'a>(&'a self, ctx: &'a Ctx) -> LinkedListIter<'a> {
        LinkedListIter {
            ctx,
            current: *self.head(),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::{Block, Operation, Value, attr::AttributeMap};
//     use proptest::prelude::*;

//     fn dummy(src: Value, dst: Value) -> Operation {
//         Operation {
//             name: "test.dummy",
//             operands: vec![src],
//             blocks: Vec::new(),
//             result: Some(dst),
//             attributes: AttributeMap::new(),
//             behind: None,
//             ahead: None,
//         }
//     }

//     fn val() -> Value {
//         Value::new(None)
//     }

//     #[test]
//     fn push_updates_head_tail() {
//         let mut bl = Block::new();
//         let mut ctx = Ctx::new();

//         assert_eq!(*bl.head(), None);
//         assert_eq!(*bl.tail(), None);

//         let ptr = bl.push(&mut ctx, dummy(val(), val()));

//         assert_eq!(*bl.head(), Some(ptr));
//         assert_eq!(*bl.tail(), Some(ptr));
//     }

//     #[test]
//     fn forward_and_backward_traversal() {
//         let mut bl = Block::new();
//         let mut ctx = Ctx::new();
//         let ptr1 = bl.push(&mut ctx, dummy(val(), val()));
//         let ptr2 = bl.push(&mut ctx, dummy(val(), val()));
//         let ptr3 = bl.push(&mut ctx, dummy(val(), val()));

//         // Forward traversal
//         let ptrs: Vec<_> = bl.iter(&ctx).map(|n| n.ahead()).collect();
//         assert_eq!(ptrs.len(), 3);
//         // The first node's ahead is Some(ptr2), second is Some(ptr3), third is None
//         assert_eq!(ctx.get_operation(OperationId(ptr1.idx)).ahead(), Some(ptr2));
//         assert_eq!(ctx.get_operation(OperationId(ptr2.idx)).ahead(), Some(ptr3));
//         assert_eq!(ctx.get_operation(OperationId(ptr3.idx)).ahead(), None);

//         // Backward traversal
//         assert_eq!(ctx.get_operation(OperationId(ptr3.idx)).behind(), Some(ptr2));
//         assert_eq!(ctx.get_operation(OperationId(ptr2.idx)).behind(), Some(ptr1));
//         assert_eq!(ctx.get_operation(OperationId(ptr1.idx)).behind(), None);
//     }

//     #[test]
//     fn insert_behind_head_and_tail() {
//         let mut bl = Block::new();
//         let mut ctx = Ctx::new();
//         let ptr1 = bl.push(&mut ctx, dummy(val(), val()));
//         let ptr2 = bl.push(&mut ctx, dummy(val(), val()));
//         let ptr3 = bl.insert_behind(&mut ctx, ptr2, dummy(val(), val()));
//         // ptr3 should be between ptr1 and ptr2
//         assert_eq!(ctx.get_operation(OperationId(ptr1.idx)).ahead(), Some(ptr3));
//         assert_eq!(ctx.get_operation(OperationId(ptr3.idx)).ahead(), Some(ptr2));
//         assert_eq!(ctx.get_operation(OperationId(ptr2.idx)).behind(), Some(ptr3));
//         assert_eq!(ctx.get_operation(OperationId(ptr3.idx)).behind(), Some(ptr1));
//     }

//     #[test]
//     fn empty_and_single_element_list() {
//         let mut bl = Block::new();
//         let mut ctx = Ctx::new();
//         assert!(bl.head().is_none());
//         assert!(bl.tail().is_none());
//         let ptr = bl.push(&mut ctx, dummy(val(), val()));
//         assert_eq!(bl.head(), bl.tail());
//         assert_eq!(ctx.get_operation(OperationId(ptr.idx)).ahead(), None);
//         assert_eq!(ctx.get_operation(OperationId(ptr.idx)).behind(), None);
//     }

//     #[test]
//     fn consistency_of_pointers_after_multiple_ops() {
//         let mut bl = Block::new();
//         let mut ctx = Ctx::new();
//         let ptrs: Vec<_> = (0..10).map(|_| bl.push(&mut ctx, dummy(val(), val()))).collect();
//         // Check forward
//         for i in 0..9 {
//             assert_eq!(ctx.get_operation(OperationId(ptrs[i].idx)).ahead(), Some(ptrs[i + 1]));
//         }
//         assert_eq!(ctx.get_operation(OperationId(ptrs[9].idx)).ahead(), None);
//         // Check backward
//         for i in 1..10 {
//             assert_eq!(ctx.get_operation(OperationId(ptrs[i].idx)).behind(), Some(ptrs[i - 1]));
//         }
//         assert_eq!(ctx.get_operation(OperationId(ptrs[0].idx)).behind(), None);
//     }

//     proptest! {
//         #[test]
//         fn push_many(count in 0usize..10000) {
//             let mut bl = Block::new();
//             let mut ctx = Ctx::new();

//             for _ in 0..count {
//                 let _ = bl.push(&mut ctx, dummy(val(), val()));
//             }

//             prop_assert_eq!(bl.len(), count);
//         }
//     }
// }
