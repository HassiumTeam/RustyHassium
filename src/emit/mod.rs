use std::collections::VecDeque;

use crate::{
    parser::{AstNode, BinOpType, FuncParams, UnaryOpType},
    runtime::{CodeObj, VMInstruction},
};

pub struct EmitContext {
    code_obj_stack: VecDeque<CodeObj>,
    label_index: u32,
}

impl EmitContext {
    fn add_inst(&mut self, inst: VMInstruction) {
        let code_obj: &mut CodeObj = self
            .code_obj_stack
            .get_mut(self.code_obj_stack.len())
            .unwrap();
        code_obj.instructions.push(inst);
    }

    fn create_label(&mut self) -> u32 {
        self.label_index += 1;
        return self.label_index;
    }

    fn place_label(&mut self, label: u32) {
        let code_obj: &mut CodeObj = self
            .code_obj_stack
            .get_mut(self.code_obj_stack.len())
            .unwrap();
        code_obj
            .labels
            .insert(label, code_obj.instructions.len() as u32 - 1);
    }

    fn place_label_here(&mut self) -> u32 {
        let code_obj: &mut CodeObj = self
            .code_obj_stack
            .get_mut(self.code_obj_stack.len())
            .unwrap();
        self.label_index += 1;
        code_obj
            .labels
            .insert(self.label_index, code_obj.instructions.len() as u32 - 1);
        return self.label_index;
    }
}

pub fn build_module(ast: AstNode) {
    let mut context: EmitContext = EmitContext {
        code_obj_stack: VecDeque::new(),
        label_index: 0,
    };
    visit(&mut context, ast);
}

fn visit(context: &mut EmitContext, node: AstNode) {
    let node_clone = node.clone();
    match node {
        AstNode::Block { children } => visit_block(context, *children),
        AstNode::Break => visit_break(context),
        AstNode::Class {
            name: _,
            extends: _,
            body: _,
        } => visit_class(context, node_clone),
        AstNode::Continue => visit_continue(context),
        AstNode::Empty => visit_empty(context),
        AstNode::For {
            initial,
            condition,
            repeated,
            body,
        } => visit_for(context, *initial, *condition, *repeated, *body),
        AstNode::Foreach {
            var: _,
            target: _,
            body: _,
        } => visit_foreach(context, node_clone),
        AstNode::Func {
            name: _,
            params: _,
            return_type: _,
            body: _,
        } => visit_func(context, node_clone),
        AstNode::If {
            predicate,
            body,
            else_body,
        } => visit_if(context, *predicate, *body, *else_body),
        AstNode::Import { target: _ } => visit_import(context, node_clone),
        AstNode::Raise { value } => visit_raise(context, *value),
        AstNode::Return { value } => visit_return(context, *value),
        AstNode::Super { args } => visit_super(context, *args),
        AstNode::TryCatch {
            try_body: _,
            value: _,
            catch_body: _,
        } => visit_try_catch(context, node_clone),
        AstNode::While { condition, body } => visit_while(context, *condition, *body),
        AstNode::ExpressionStatement { expression } => {
            visit_expression_statement(context, *expression)
        }
        AstNode::Assign { left: _, right: _ } => visit_assign(context, node_clone),
        AstNode::AttribAccess { target, attrib } => visit_attrib_access(context, *target, attrib),
        AstNode::BinOp { op, left, right } => visit_bin_op(context, op, *left, *right),
        AstNode::Id { value } => visit_id(context, value),
        AstNode::Invoke { target, args } => visit_invoke(context, *target, *args),
        AstNode::Number { value } => visit_number(context, value),
        AstNode::String { value } => visit_string(context, value),
        AstNode::Subscript { target, key } => visit_subscript(context, *key, *target),
        AstNode::UnaryOp { op, target } => visit_unary_op(context, op, *target),
    }
}

fn visit_block(context: &mut EmitContext, children: Vec<AstNode>) {
    for child in children {
        visit(context, child)
    }
}
fn visit_break(context: &mut EmitContext) {
    context.add_inst(VMInstruction::Break);
}
fn visit_class(context: &mut EmitContext, node: AstNode) {}
fn visit_continue(context: &mut EmitContext) {
    context.add_inst(VMInstruction::Continue);
}
fn visit_empty(context: &mut EmitContext) {}
fn visit_for(
    context: &mut EmitContext,
    initial: AstNode,
    condition: AstNode,
    repeated: AstNode,
    body: AstNode,
) {
    let end_label = context.create_label();
    let body_label = context.create_label();
    visit(context, initial);
    context.place_label(body_label);
    visit(context, condition);
    context.add_inst(VMInstruction::JumpIfFalse { to: end_label });
    visit(context, body);
    visit(context, repeated);
    context.add_inst(VMInstruction::Jump { to: body_label });
    context.place_label(end_label);
}
fn visit_foreach(context: &mut EmitContext, node: AstNode) {}
fn visit_func(context: &mut EmitContext, node: AstNode) {}
fn visit_if(
    context: &mut EmitContext,
    predicate: AstNode,
    body: AstNode,
    else_body: Option<AstNode>,
) {
    let else_label = context.create_label();
    visit(context, predicate);
    context.add_inst(VMInstruction::JumpIfFalse { to: else_label });
    visit(context, body);
    context.place_label(else_label);
    if else_body.is_some() {
        visit(context, else_body.unwrap());
    }
}
fn visit_import(context: &mut EmitContext, node: AstNode) {}
fn visit_raise(context: &mut EmitContext, value: AstNode) {
    visit(context, value);
    context.add_inst(VMInstruction::Raise);
}
fn visit_return(context: &mut EmitContext, value: AstNode) {
    visit(context, value);
    context.add_inst(VMInstruction::Return)
}
fn visit_super(context: &mut EmitContext, _args: Vec<AstNode>) {
    let mut args = _args.clone();
    args.reverse();
    for arg in args {
        visit(context, arg.to_owned());
    }
    context.add_inst(VMInstruction::Super {
        arg_count: _args.len() as u32,
    });
}
fn visit_try_catch(context: &mut EmitContext, node: AstNode) {}
fn visit_while(context: &mut EmitContext, condition: AstNode, body: AstNode) {
    let body_label = context.create_label();
    let end_label = context.create_label();
    context.place_label(body_label);
    visit(context, condition);
    context.add_inst(VMInstruction::JumpIfFalse { to: end_label });
    visit(context, body);
    context.add_inst(VMInstruction::Jump { to: body_label });
    context.place_label(end_label);
}
fn visit_expression_statement(context: &mut EmitContext, expression: AstNode) {
    visit(context, expression);
    context.add_inst(VMInstruction::Pop);
}
fn visit_assign(context: &mut EmitContext, node: AstNode) {}
fn visit_attrib_access(context: &mut EmitContext, target: AstNode, attrib: String) {
    visit(context, target);
    context.add_inst(VMInstruction::LoadAttrib { attrib });
}
fn visit_bin_op(context: &mut EmitContext, op: BinOpType, left: AstNode, right: AstNode) {
    visit(context, right);
    visit(context, left);
    context.add_inst(VMInstruction::BinOp { op });
}
fn visit_id(context: &mut EmitContext, value: String) {
    context.add_inst(VMInstruction::LoadId { id: value });
}
fn visit_invoke(context: &mut EmitContext, target: AstNode, _args: Vec<AstNode>) {
    let mut args = _args.clone();
    args.reverse();
    for arg in args {
        visit(context, arg.to_owned());
    }
    visit(context, target);
    context.add_inst(VMInstruction::Invoke {
        arg_count: _args.len() as u32,
    });
}
fn visit_number(context: &mut EmitContext, value: f64) {
    context.add_inst(VMInstruction::LoadNumber { value });
}
fn visit_string(context: &mut EmitContext, value: String) {
    context.add_inst(VMInstruction::LoadString { value });
}
fn visit_subscript(context: &mut EmitContext, key: AstNode, target: AstNode) {
    visit(context, key);
    visit(context, target);
    context.add_inst(VMInstruction::LoadSubscript);
}
fn visit_unary_op(context: &mut EmitContext, op: UnaryOpType, target: AstNode) {
    visit(context, target);
    context.add_inst(VMInstruction::UnaryOp { op });
}
