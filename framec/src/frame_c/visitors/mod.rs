pub mod cpp_visitor;
pub mod cs_visitor;
pub mod cs_visitor_for_bob;
pub mod gdscript_3_2_visitor;
pub mod java_8_visitor;
pub mod javascript_visitor;
pub mod plantuml_visitor;
pub mod python_visitor;
pub mod rust_visitor;
pub mod smcat_visitor;
//pub mod xtate_visitor;

use super::ast::*;

#[rustfmt::skip]
pub trait AstVisitor {
    fn visit_system_node(&mut self, _node: &SystemNode) {}
    fn visit_interface_block_node(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_method_node(&mut self, _node: &InterfaceMethodNode) {}
    fn visit_machine_block_node(&mut self, _node: &MachineBlockNode) {}
    fn visit_state_node(&mut self, _node: &StateNode) {}
    fn visit_event_handler_node(&mut self, _node: &EventHandlerNode) {}
    fn visit_event_handler_terminator_node(&mut self, _node: &TerminatorExpr) {}
    fn visit_call_statement_node(&mut self, _node: &CallStmtNode) {}
    fn visit_frame_messages_enum(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_parameters(&mut self, _node: &InterfaceBlockNode) {}
    fn visit_interface_method_call_expression_node(&mut self, _node: &InterfaceMethodCallExprNode) {}
    fn visit_interface_method_call_expression_node_to_string(&mut self, _node: &InterfaceMethodCallExprNode, _output: &mut String) {}
    fn visit_call_expression_node(&mut self, _node: &CallExprNode) {}
    fn visit_call_expression_node_to_string(&mut self, _node: &CallExprNode, _output: &mut String) {}
    fn visit_call_expr_list_node(&mut self, _node: &CallExprListNode) {}
    fn visit_call_expr_list_node_to_string(&mut self, _node: &CallExprListNode, _output: &mut String) {}
    fn visit_call_chain_literal_expr_node(&mut self, _node: &CallChainLiteralExprNode) {}
    fn visit_call_chain_literal_expr_node_to_string(&mut self, _node: &CallChainLiteralExprNode, _output: &mut String) {}
    fn visit_call_chain_literal_statement_node(&mut self, _node: &CallChainLiteralStmtNode) {}
    fn visit_transition_statement_node(&mut self, _node: &TransitionStatementNode) {}
    fn visit_state_ref_node(&mut self, _node: &StateRefNode) {}
    fn visit_parameter_node(&mut self, _node: &ParameterNode) {}
    fn visit_dispatch_node(&mut self, _node: &DispatchNode) {}
    fn visit_test_statement_node(&mut self, _node: &TestStatementNode) {}
    fn visit_bool_test_node(&mut self, _node: &BoolTestNode) {}
    fn visit_bool_test_conditional_branch_node(&mut self, _node: &BoolTestConditionalBranchNode) {}
    fn visit_bool_test_else_branch_node(&mut self, _node: &BoolTestElseBranchNode) {}
    fn visit_string_match_test_node(&mut self, _node: &StringMatchTestNode) {}
    fn visit_string_match_test_match_branch_node(&mut self, _node: &StringMatchTestMatchBranchNode) {}
    fn visit_string_match_test_else_branch_node(&mut self, _node: &StringMatchTestElseBranchNode) {}
    fn visit_string_match_test_pattern_node(&mut self, _node: &StringMatchTestPatternNode) {}
    fn visit_number_match_test_node(&mut self, _node: &NumberMatchTestNode) {}
    fn visit_number_match_test_match_branch_node(&mut self, _node: &NumberMatchTestMatchBranchNode) {}
    fn visit_number_match_test_else_branch_node(&mut self, _node: &NumberMatchTestElseBranchNode) {}
    fn visit_number_match_test_pattern_node(&mut self, _node: &NumberMatchTestPatternNode) {}
    fn visit_expression_list_node(&mut self, _expr_list: &ExprListNode) {}
    fn visit_expression_list_node_to_string(&mut self, _expr_list: &ExprListNode, _output: &mut String) {}
    fn visit_literal_expression_node(&mut self, _node: &LiteralExprNode) {}
    fn visit_literal_expression_node_to_string(&mut self, _node: &LiteralExprNode, _output: &mut String) {}
    fn visit_identifier_node(&mut self, _node: &IdentifierNode) {}
    fn visit_identifier_node_to_string(&mut self, _node: &IdentifierNode, _output: &mut String) {}
    fn visit_state_stack_operation_node(&mut self, _node: &StateStackOperationNode) {}
    fn visit_state_stack_operation_node_to_string(&mut self, _node: &StateStackOperationNode, _output: &mut String) {}
    fn visit_state_stack_operation_statement_node(&mut self, _node: &StateStackOperationStatementNode) {}
    fn visit_state_context_node(&mut self, _node: &StateContextNode) {}
    fn visit_change_state_statement_node(&mut self, _node: &ChangeStateStatementNode) {}
    fn visit_frame_event_part(&mut self, _event_part: &FrameEventPart) {}
    fn visit_frame_event_part_to_string(&mut self, _event_part: &FrameEventPart, _output: &mut String) {}
    fn visit_actions_block_node(&mut self, _node: &ActionsBlockNode) {}
    fn visit_action_node_rust_trait(&mut self, _node: &ActionsBlockNode) {}
    fn visit_actions_node_rust_impl(&mut self, _node: &ActionsBlockNode) {}
    fn visit_action_decl_node(&mut self, _node: &ActionNode) {}
    fn visit_action_impl_node(&mut self, _node: &ActionNode) {}
    fn visit_action_call_expression_node(&mut self, _node: &ActionCallExprNode) {}
    fn visit_action_call_expression_node_to_string(&mut self, _node: &ActionCallExprNode, _output: &mut String) {}
    fn visit_action_call_statement_node(&mut self, _node: &ActionCallStmtNode) {}
    fn visit_domain_block_node(&mut self, _node: &DomainBlockNode) {}
    fn visit_domain_variable_decl_node(&mut self, _node: &VariableDeclNode) {}
    fn visit_variable_decl_node(&mut self, _node: &VariableDeclNode) {}
    fn visit_variable_expr_node(&mut self, _node: &VariableNode) {}
    fn visit_variable_expr_node_to_string(&mut self, _node: &VariableNode, _output: &mut String) {}
    fn visit_variable_stmt_node(&mut self, _node: &VariableStmtNode) {}
    fn visit_assignment_expr_node(&mut self, _node: &AssignmentExprNode) {}
    fn visit_assignment_expr_node_to_string(&mut self, _node: &AssignmentExprNode, _output: &mut String) {}
    fn visit_assignment_statement_node(&mut self, _node: &AssignmentStmtNode) {}
    fn visit_unary_expr_node(&mut self, _node: &UnaryExprNode) {}
    fn visit_unary_expr_node_to_string(&mut self, _node: &UnaryExprNode, _output: &mut String) {}
    fn visit_binary_expr_node(&mut self, _node: &BinaryExprNode) {}
    fn visit_binary_expr_node_to_string(&mut self, _node: &BinaryExprNode, _output: &mut String) {}
    fn visit_operator_type(&mut self, _operator_type: &OperatorType) {}
    fn visit_operator_type_to_string(&mut self, _operator_type: &OperatorType, _output: &mut String) {}
}
