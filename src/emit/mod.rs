use crate::parser::{AstNode, BinOpType, FuncParams, UnaryOpType};

struct EmitContext {}

pub fn build_module(ast: AstNode) {
    let mut context: EmitContext = EmitContext {};
    visit(&mut context, ast);
}

fn visit(context: &mut EmitContext, node: AstNode) {
    let node_clone = node.clone();
    match node {
        AstNode::Block { children: _ } => visit_block(context, node_clone),
        AstNode::Break => visit_break(context, node_clone),
        AstNode::Class {
            name: _,
            extends: _,
            body: _,
        } => visit_class(context, node_clone),
        AstNode::Continue => visit_continue(context, node_clone),
        AstNode::Empty => visit_empty(context, node_clone),
        AstNode::For {
            initial: _,
            condition: _,
            repeated: _,
            body: _,
        } => visit_for(context, node_clone),
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
            predicate: _,
            body: _,
            else_body: _,
        } => visit_if(context, node_clone),
        AstNode::Import { target: _ } => visit_import(context, node_clone),
        AstNode::Raise { value: _ } => visit_raise(context, node_clone),
        AstNode::Return { value: _ } => visit_return(context, node_clone),
        AstNode::Super { args: _ } => visit_super(context, node_clone),
        AstNode::TryCatch {
            try_body: _,
            value: _,
            catch_body: _,
        } => visit_try_catch(context, node_clone),
        AstNode::While {
            condition: _,
            body: _,
        } => visit_while(context, node_clone),
        AstNode::ExpressionStatement { expression: _ } => {
            visit_expression_statement(context, node_clone)
        }
        AstNode::Assign { left: _, right: _ } => visit_assign(context, node_clone),
        AstNode::AttribAccess {
            target: _,
            attrib: _,
        } => visit_attrib_access(context, node_clone),
        AstNode::BinOp {
            op: _,
            left: _,
            right: _,
        } => visit_bin_op(context, node_clone),
        AstNode::Id { value: _ } => visit_id(context, node_clone),
        AstNode::Invoke { target: _, args: _ } => visit_invoke(context, node_clone),
        AstNode::Number { value: _ } => visit_number(context, node_clone),
        AstNode::String { value: _ } => visit_string(context, node_clone),
        AstNode::Subscript { target: _, key: _ } => visit_subscript(context, node_clone),
        AstNode::UnaryOp { op: _, target: _ } => visit_unary_op(context, node_clone),
    }
}

fn visit_block(context: &mut EmitContext, node: AstNode) {}
fn visit_break(context: &mut EmitContext, node: AstNode) {}
fn visit_class(context: &mut EmitContext, node: AstNode) {}
fn visit_continue(context: &mut EmitContext, node: AstNode) {}
fn visit_empty(context: &mut EmitContext, node: AstNode) {}
fn visit_for(context: &mut EmitContext, node: AstNode) {}
fn visit_foreach(context: &mut EmitContext, node: AstNode) {}
fn visit_func(context: &mut EmitContext, node: AstNode) {}
fn visit_if(context: &mut EmitContext, node: AstNode) {}
fn visit_import(context: &mut EmitContext, node: AstNode) {}
fn visit_raise(context: &mut EmitContext, node: AstNode) {}
fn visit_return(context: &mut EmitContext, node: AstNode) {}
fn visit_super(context: &mut EmitContext, node: AstNode) {}
fn visit_try_catch(context: &mut EmitContext, node: AstNode) {}
fn visit_while(context: &mut EmitContext, node: AstNode) {}
fn visit_expression_statement(context: &mut EmitContext, node: AstNode) {}
fn visit_assign(context: &mut EmitContext, node: AstNode) {}
fn visit_attrib_access(context: &mut EmitContext, node: AstNode) {}
fn visit_bin_op(context: &mut EmitContext, node: AstNode) {}
fn visit_id(context: &mut EmitContext, node: AstNode) {}
fn visit_invoke(context: &mut EmitContext, node: AstNode) {}
fn visit_number(context: &mut EmitContext, node: AstNode) {}
fn visit_string(context: &mut EmitContext, node: AstNode) {}
fn visit_subscript(context: &mut EmitContext, node: AstNode) {}
fn visit_unary_op(context: &mut EmitContext, node: AstNode) {}
