use blanket::blanket;

use crate::ast_core as ast;

#[blanket(default = "visit")]
pub(crate) trait AstVisitor {
    fn visit_transform(&mut self, transform: &mut ast::Transform);
    fn visit_declaration(&mut self, declaration: &mut ast::Declaration);
    fn visit_accumulator(&mut self, accumulator: &mut ast::Accumulator);
    fn visit_accumulator_rule(&mut self, rule: &mut ast::AccumulatorRule);
    fn visit_character_map(&mut self, character_map: &mut ast::CharacterMap);
    fn visit_decimal_format(&mut self, decimal_format: &mut ast::DecimalFormat);
    fn visit_output_character(&mut self, output_character: &mut ast::OutputCharacter);
    fn visit_function(&mut self, function: &mut ast::Function);
    fn visit_param(&mut self, param: &mut ast::Param);
    fn visit_global_context_item(&mut self, global_context_item: &mut ast::GlobalContextItem);
    fn visit_import(&mut self, import: &mut ast::Import);
    fn visit_import_schema(&mut self, import_schema: &mut ast::ImportSchema);
    fn visit_include(&mut self, include: &mut ast::Include);
    fn visit_key(&mut self, key: &mut ast::Key);
    fn visit_mode(&mut self, mode: &mut ast::Mode);
    fn visit_namespace_alias(&mut self, namespace_alias: &mut ast::NamespaceAlias);
    fn visit_output(&mut self, output: &mut ast::Output);
    fn visit_preserve_space(&mut self, preserve_space: &mut ast::PreserveSpace);
    fn visit_strip_space(&mut self, strip_space: &mut ast::StripSpace);
    fn visit_template(&mut self, template: &mut ast::Template);
    fn visit_use_package(&mut self, use_package: &mut ast::UsePackage);
    fn visit_variable(&mut self, variable: &mut ast::Variable);
    fn visit_sequence_constructor(&mut self, sequence_constructor: &mut ast::SequenceConstructor);
    fn visit_sequence_constructor_item(&mut self, item: &mut ast::SequenceConstructorItem);
    fn visit_context_item(&mut self, context_item: &mut ast::ContextItem);
    fn visit_accept(&mut self, accept: &mut ast::Accept);
    fn visit_override(&mut self, override_: &mut ast::Override);
    fn visit_attribute_set(&mut self, attribute_set: &mut ast::AttributeSet);
    fn visit_attribute(&mut self, attribute: &mut ast::Attribute);
    fn visit_content_element(&mut self, element_node: &mut ast::ElementNode);
    fn visit_instruction(&mut self, instruction: &mut ast::SequenceConstructorInstruction);
    fn visit_analyze_string(&mut self, analyze_string: &mut ast::AnalyzeString);
    fn visit_apply_imports(&mut self, apply_imports: &mut ast::ApplyImports);
    fn visit_apply_templates(&mut self, apply_templates: &mut ast::ApplyTemplates);
    fn visit_assert(&mut self, assert: &mut ast::Assert);
    fn visit_break(&mut self, break_: &mut ast::Break);
    fn visit_call_template(&mut self, call_template: &mut ast::CallTemplate);
    fn visit_choose(&mut self, choose: &mut ast::Choose);
    fn visit_comment(&mut self, comment: &mut ast::Comment);
    fn visit_copy(&mut self, copy: &mut ast::Copy);
    fn visit_copy_of(&mut self, copy_of: &mut ast::CopyOf);
    fn visit_document(&mut self, document: &mut ast::Document);
    fn visit_element(&mut self, element: &mut ast::Element);
    fn visit_evaluate(&mut self, evaluate: &mut ast::Evaluate);
    fn visit_fallback(&mut self, fallback: &mut ast::Fallback);
    fn visit_for_each(&mut self, for_each: &mut ast::ForEach);
    fn visit_for_each_group(&mut self, for_each_group: &mut ast::ForEachGroup);
    fn visit_fork(&mut self, fork: &mut ast::Fork);
    fn visit_if(&mut self, if_: &mut ast::If);
    fn visit_iterate(&mut self, iterate: &mut ast::Iterate);
    fn visit_map(&mut self, map: &mut ast::Map);
    fn visit_map_entry(&mut self, map_entry: &mut ast::MapEntry);
    fn visit_merge(&mut self, merge: &mut ast::Merge);
    fn visit_message(&mut self, message: &mut ast::Message);
    fn visit_namespace(&mut self, namespace: &mut ast::Namespace);
    fn visit_next_iteration(&mut self, next_iteration: &mut ast::NextIteration);
    fn visit_next_match(&mut self, next_match: &mut ast::NextMatch);
    fn visit_number(&mut self, number: &mut ast::Number);
    fn visit_on_empty(&mut self, on_empty: &mut ast::OnEmpty);
    fn visit_on_non_empty(&mut self, on_non_empty: &mut ast::OnNonEmpty);
    fn visit_perform_sort(&mut self, perform_sort: &mut ast::PerformSort);
    fn visit_processing_instruction(
        &mut self,
        processing_instruction: &mut ast::ProcessingInstruction,
    );
    fn visit_result_document(&mut self, result_document: &mut ast::ResultDocument);
    fn visit_sequence(&mut self, sequence: &mut ast::Sequence);
    fn visit_source_document(&mut self, source_document: &mut ast::SourceDocument);
    fn visit_text(&mut self, text: &mut ast::Text);
    fn visit_try(&mut self, try_: &mut ast::Try);
    fn visit_value_of(&mut self, value_of: &mut ast::ValueOf);
    fn visit_where_populated(&mut self, where_populated: &mut ast::WherePopulated);
    fn visit_matching_substring(&mut self, matching_substring: &mut ast::MatchingSubstring);
    fn visit_non_matching_substring(
        &mut self,
        non_matching_substring: &mut ast::NonMatchingSubstring,
    );
    fn visit_with_param(&mut self, with_param: &mut ast::WithParam);
    fn visit_sort(&mut self, sort: &mut ast::Sort);
    fn visit_when(&mut self, when: &mut ast::When);
    fn visit_otherwise(&mut self, otherwise: &mut ast::Otherwise);
    fn visit_on_completion(&mut self, on_completion: &mut ast::OnCompletion);
    fn visit_merge_action(&mut self, merge_action: &mut ast::MergeAction);
    fn visit_merge_key(&mut self, merge_key: &mut ast::MergeKey);
    fn visit_merge_source(&mut self, merge_source: &mut ast::MergeSource);
    fn visit_catch(&mut self, catch: &mut ast::Catch);
}

pub(crate) mod visit {
    use super::AstVisitor;
    use crate::ast_core as ast;

    pub(crate) fn visit_transform<V: AstVisitor + ?Sized>(
        v: &mut V,
        transform: &mut ast::Transform,
    ) {
        for declaration in transform.declarations.iter_mut() {
            v.visit_declaration(declaration)
        }
    }

    pub(crate) fn visit_declaration<V: AstVisitor + ?Sized>(
        v: &mut V,
        declaration: &mut ast::Declaration,
    ) {
        use ast::Declaration::*;
        match declaration {
            Accumulator(accumulator) => v.visit_accumulator(accumulator),
            CharacterMap(character_map) => v.visit_character_map(character_map),
            DecimalFormat(decimal_format) => v.visit_decimal_format(decimal_format),
            Function(function) => v.visit_function(function),
            GlobalContextItem(global_context_item) => {
                v.visit_global_context_item(global_context_item)
            }
            Import(import) => v.visit_import(import),
            ImportSchema(import_schema) => v.visit_import_schema(import_schema),
            Include(include) => v.visit_include(include),
            Key(key) => v.visit_key(key),
            Mode(mode) => v.visit_mode(mode),
            NamespaceAlias(namespace_alias) => v.visit_namespace_alias(namespace_alias),
            Output(output) => v.visit_output(output),
            Param(param) => v.visit_param(param),
            PreserveSpace(preserve_space) => v.visit_preserve_space(preserve_space),
            StripSpace(strip_space) => v.visit_strip_space(strip_space),
            Template(template) => v.visit_template(template),
            UsePackage(use_package) => v.visit_use_package(use_package),
            Variable(variable) => v.visit_variable(variable),
        }
    }

    pub(crate) fn visit_accumulator<V: AstVisitor + ?Sized>(
        v: &mut V,
        accumulator: &mut ast::Accumulator,
    ) {
        for rule in accumulator.rules.iter_mut() {
            v.visit_accumulator_rule(rule)
        }
    }

    pub(crate) fn visit_accumulator_rule<V: AstVisitor + ?Sized>(
        v: &mut V,
        rule: &mut ast::AccumulatorRule,
    ) {
        v.visit_sequence_constructor(&mut rule.sequence_constructor)
    }

    pub(crate) fn visit_character_map<V: AstVisitor + ?Sized>(
        v: &mut V,
        character_map: &mut ast::CharacterMap,
    ) {
        for output_character in character_map.output_characters.iter_mut() {
            v.visit_output_character(output_character)
        }
    }

    pub(crate) fn visit_output_character<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _output_character: &mut ast::OutputCharacter,
    ) {
        // no children
    }

    pub(crate) fn visit_decimal_format<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _decimal_format: &mut ast::DecimalFormat,
    ) {
        // no children
    }

    pub(crate) fn visit_function<V: AstVisitor + ?Sized>(v: &mut V, function: &mut ast::Function) {
        for param in function.params.iter_mut() {
            v.visit_param(param)
        }
        v.visit_sequence_constructor(&mut function.sequence_constructor)
    }

    pub(crate) fn visit_param<V: AstVisitor + ?Sized>(v: &mut V, param: &mut ast::Param) {
        v.visit_sequence_constructor(&mut param.sequence_constructor)
    }

    pub(crate) fn visit_global_context_item<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _global_context_item: &mut ast::GlobalContextItem,
    ) {
        // no children
    }

    pub(crate) fn visit_import<V: AstVisitor + ?Sized>(_v: &mut V, _import: &mut ast::Import) {
        // no children
    }

    pub(crate) fn visit_import_schema<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _import_schema: &mut ast::ImportSchema,
    ) {
        // no children
    }

    pub(crate) fn visit_include<V: AstVisitor + ?Sized>(_v: &mut V, _include: &mut ast::Include) {
        // no children
    }

    pub(crate) fn visit_key<V: AstVisitor + ?Sized>(v: &mut V, key: &mut ast::Key) {
        v.visit_sequence_constructor(&mut key.sequence_constructor)
    }

    pub(crate) fn visit_mode<V: AstVisitor + ?Sized>(_v: &mut V, _mode: &mut ast::Mode) {
        // no children
    }

    pub(crate) fn visit_namespace_alias<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _namespace_alias: &mut ast::NamespaceAlias,
    ) {
        // no children
    }

    pub(crate) fn visit_output<V: AstVisitor + ?Sized>(_v: &mut V, _output: &mut ast::Output) {
        // no children
    }

    pub(crate) fn visit_preserve_space<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _preserve_space: &mut ast::PreserveSpace,
    ) {
        // no children
    }

    pub(crate) fn visit_strip_space<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _strip_space: &mut ast::StripSpace,
    ) {
        // no children
    }

    pub(crate) fn visit_template<V: AstVisitor + ?Sized>(v: &mut V, template: &mut ast::Template) {
        if let Some(context_item) = &mut template.context_item {
            v.visit_context_item(context_item)
        }
        for param in template.params.iter_mut() {
            v.visit_param(param)
        }
        v.visit_sequence_constructor(&mut template.sequence_constructor)
    }

    pub(crate) fn visit_use_package<V: AstVisitor + ?Sized>(
        v: &mut V,
        use_package: &mut ast::UsePackage,
    ) {
        for item in use_package.content.iter_mut() {
            match item {
                ast::UsePackageContent::Accept(accept) => v.visit_accept(accept),
                ast::UsePackageContent::Override(override_) => v.visit_override(override_),
            }
        }
    }

    pub(crate) fn visit_accept<V: AstVisitor + ?Sized>(_v: &mut V, _accept: &mut ast::Accept) {
        // no children
    }

    pub(crate) fn visit_override<V: AstVisitor + ?Sized>(v: &mut V, override_: &mut ast::Override) {
        for item in override_.content.iter_mut() {
            match item {
                ast::OverrideContent::Template(template) => v.visit_template(template),
                ast::OverrideContent::Function(function) => v.visit_function(function),
                ast::OverrideContent::Variable(variable) => v.visit_variable(variable),
                ast::OverrideContent::Param(param) => v.visit_param(param),
                ast::OverrideContent::AttributeSet(attribute_set) => {
                    v.visit_attribute_set(attribute_set)
                }
            }
        }
    }

    pub(crate) fn visit_attribute_set<V: AstVisitor + ?Sized>(
        v: &mut V,
        attribute_set: &mut ast::AttributeSet,
    ) {
        for attribute in attribute_set.attributes.iter_mut() {
            v.visit_attribute(attribute)
        }
    }

    pub(crate) fn visit_attribute<V: AstVisitor + ?Sized>(
        v: &mut V,
        attribute: &mut ast::Attribute,
    ) {
        v.visit_sequence_constructor(&mut attribute.sequence_constructor)
    }

    pub(crate) fn visit_variable<V: AstVisitor + ?Sized>(v: &mut V, variable: &mut ast::Variable) {
        v.visit_sequence_constructor(&mut variable.sequence_constructor)
    }

    pub(crate) fn visit_context_item<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _context_item: &mut ast::ContextItem,
    ) {
        // no children
    }

    pub(crate) fn visit_sequence_constructor<V: AstVisitor + ?Sized>(
        v: &mut V,
        sequence_constructor: &mut ast::SequenceConstructor,
    ) {
        for item in sequence_constructor.iter_mut() {
            v.visit_sequence_constructor_item(item)
        }
    }

    pub(crate) fn visit_sequence_constructor_item<V: AstVisitor + ?Sized>(
        v: &mut V,
        sequence_constructor_item: &mut ast::SequenceConstructorItem,
    ) {
        use ast::SequenceConstructorItem::*;

        match sequence_constructor_item {
            Content(ast::Content::Element(element)) => v.visit_content_element(element),
            // TODO: document content
            Content(_) => {
                // no children
            }
            Instruction(instruction) => v.visit_instruction(instruction),
        }
    }

    pub(crate) fn visit_content_element<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _element_node: &mut ast::ElementNode,
    ) {
        // TODO
    }

    pub(crate) fn visit_instruction<V: AstVisitor + ?Sized>(
        v: &mut V,
        instruction: &mut ast::SequenceConstructorInstruction,
    ) {
        use ast::SequenceConstructorInstruction::*;
        match instruction {
            AnalyzeString(analyze_string) => v.visit_analyze_string(analyze_string),
            ApplyImports(apply_imports) => v.visit_apply_imports(apply_imports),
            ApplyTemplates(apply_templates) => v.visit_apply_templates(apply_templates),
            Assert(assert) => v.visit_assert(assert),
            Attribute(attribute) => v.visit_attribute(attribute),
            Break(break_) => v.visit_break(break_),
            CallTemplate(call_template) => v.visit_call_template(call_template),
            Choose(choose) => v.visit_choose(choose),
            Comment(comment) => v.visit_comment(comment),
            Copy(copy) => v.visit_copy(copy),
            CopyOf(copy_of) => v.visit_copy_of(copy_of),
            Document(document) => v.visit_document(document),
            Element(element) => v.visit_element(element),
            Evaluate(evaluate) => v.visit_evaluate(evaluate),
            Fallback(fallback) => v.visit_fallback(fallback),
            ForEach(for_each) => v.visit_for_each(for_each),
            ForEachGroup(for_each_group) => v.visit_for_each_group(for_each_group),
            Fork(fork) => v.visit_fork(fork),
            If(if_) => v.visit_if(if_),
            Iterate(iterate) => v.visit_iterate(iterate),
            Map(map) => v.visit_map(map),
            MapEntry(map_entry) => v.visit_map_entry(map_entry),
            Merge(merge) => v.visit_merge(merge),
            Message(message) => v.visit_message(message),
            Namespace(namespace) => v.visit_namespace(namespace),
            NextIteration(next_iteration) => v.visit_next_iteration(next_iteration),
            NextMatch(next_match) => v.visit_next_match(next_match),
            Number(number) => v.visit_number(number),
            OnEmpty(on_empty) => v.visit_on_empty(on_empty),
            OnNonEmpty(on_non_empty) => v.visit_on_non_empty(on_non_empty),
            PerformSort(perform_sort) => v.visit_perform_sort(perform_sort),
            ProcessingInstruction(processing_instruction) => {
                v.visit_processing_instruction(processing_instruction)
            }
            ResultDocument(result_document) => v.visit_result_document(result_document),
            Sequence(sequence) => v.visit_sequence(sequence),
            SourceDocument(source_document) => v.visit_source_document(source_document),
            Text(text) => v.visit_text(text),
            Try(try_) => v.visit_try(try_),
            ValueOf(value_of) => v.visit_value_of(value_of),
            Variable(variable) => v.visit_variable(variable),
            WherePopulated(where_populated) => v.visit_where_populated(where_populated),
        }
    }

    pub(crate) fn visit_analyze_string<V: AstVisitor + ?Sized>(
        v: &mut V,
        analyze_string: &mut ast::AnalyzeString,
    ) {
        if let Some(matching_substring) = &mut analyze_string.matching_substring {
            v.visit_matching_substring(matching_substring)
        }
        if let Some(non_matching_substring) = &mut analyze_string.non_matching_substring {
            v.visit_non_matching_substring(non_matching_substring)
        }
        for fallback in analyze_string.fallbacks.iter_mut() {
            v.visit_fallback(fallback)
        }
    }

    pub(crate) fn visit_matching_substring<V: AstVisitor + ?Sized>(
        v: &mut V,
        matching_substring: &mut ast::MatchingSubstring,
    ) {
        v.visit_sequence_constructor(&mut matching_substring.sequence_constructor)
    }

    pub(crate) fn visit_non_matching_substring<V: AstVisitor + ?Sized>(
        v: &mut V,
        non_matching_substring: &mut ast::NonMatchingSubstring,
    ) {
        v.visit_sequence_constructor(&mut non_matching_substring.sequence_constructor)
    }

    pub(crate) fn visit_apply_imports<V: AstVisitor + ?Sized>(
        v: &mut V,
        apply_imports: &mut ast::ApplyImports,
    ) {
        for with_param in apply_imports.with_params.iter_mut() {
            v.visit_with_param(with_param)
        }
    }

    pub(crate) fn visit_with_param<V: AstVisitor + ?Sized>(
        v: &mut V,
        with_param: &mut ast::WithParam,
    ) {
        v.visit_sequence_constructor(&mut with_param.sequence_constructor)
    }

    pub(crate) fn visit_apply_templates<V: AstVisitor + ?Sized>(
        v: &mut V,
        apply_templates: &mut ast::ApplyTemplates,
    ) {
        for item in apply_templates.content.iter_mut() {
            match item {
                ast::ApplyTemplatesContent::Sort(sort) => v.visit_sort(sort),
                ast::ApplyTemplatesContent::WithParam(with_param) => v.visit_with_param(with_param),
            }
        }
    }

    pub(crate) fn visit_sort<V: AstVisitor + ?Sized>(v: &mut V, sort: &mut ast::Sort) {
        v.visit_sequence_constructor(&mut sort.sequence_constructor)
    }

    pub(crate) fn visit_assert<V: AstVisitor + ?Sized>(v: &mut V, assert: &mut ast::Assert) {
        v.visit_sequence_constructor(&mut assert.sequence_constructor)
    }

    pub(crate) fn visit_break<V: AstVisitor + ?Sized>(v: &mut V, break_: &mut ast::Break) {
        v.visit_sequence_constructor(&mut break_.sequence_constructor)
    }

    pub(crate) fn visit_call_template<V: AstVisitor + ?Sized>(
        v: &mut V,
        call_template: &mut ast::CallTemplate,
    ) {
        for with_param in call_template.with_params.iter_mut() {
            v.visit_with_param(with_param)
        }
    }

    pub(crate) fn visit_choose<V: AstVisitor + ?Sized>(v: &mut V, choose: &mut ast::Choose) {
        for when in choose.when.iter_mut() {
            v.visit_when(when)
        }
        if let Some(otherwise) = &mut choose.otherwise {
            v.visit_otherwise(otherwise)
        }
    }

    pub(crate) fn visit_when<V: AstVisitor + ?Sized>(v: &mut V, when: &mut ast::When) {
        v.visit_sequence_constructor(&mut when.sequence_constructor)
    }

    pub(crate) fn visit_otherwise<V: AstVisitor + ?Sized>(
        v: &mut V,
        otherwise: &mut ast::Otherwise,
    ) {
        v.visit_sequence_constructor(&mut otherwise.sequence_constructor)
    }

    pub(crate) fn visit_comment<V: AstVisitor + ?Sized>(v: &mut V, comment: &mut ast::Comment) {
        v.visit_sequence_constructor(&mut comment.sequence_constructor)
    }

    pub(crate) fn visit_copy<V: AstVisitor + ?Sized>(v: &mut V, copy: &mut ast::Copy) {
        v.visit_sequence_constructor(&mut copy.sequence_constructor)
    }

    pub(crate) fn visit_copy_of<V: AstVisitor + ?Sized>(_v: &mut V, _copy_of: &mut ast::CopyOf) {
        // no children
    }

    pub(crate) fn visit_document<V: AstVisitor + ?Sized>(v: &mut V, document: &mut ast::Document) {
        v.visit_sequence_constructor(&mut document.sequence_constructor)
    }

    pub(crate) fn visit_element<V: AstVisitor + ?Sized>(v: &mut V, element: &mut ast::Element) {
        v.visit_sequence_constructor(&mut element.sequence_constructor)
    }

    pub(crate) fn visit_evaluate<V: AstVisitor + ?Sized>(v: &mut V, evaluate: &mut ast::Evaluate) {
        for item in evaluate.content.iter_mut() {
            match item {
                ast::EvaluateContent::WithParam(with_param) => v.visit_with_param(with_param),
                ast::EvaluateContent::Fallback(fallback) => v.visit_fallback(fallback),
            }
        }
    }

    pub(crate) fn visit_fallback<V: AstVisitor + ?Sized>(v: &mut V, fallback: &mut ast::Fallback) {
        v.visit_sequence_constructor(&mut fallback.sequence_constructor)
    }

    pub(crate) fn visit_for_each<V: AstVisitor + ?Sized>(v: &mut V, for_each: &mut ast::ForEach) {
        for sort in for_each.sort.iter_mut() {
            v.visit_sort(sort)
        }
        v.visit_sequence_constructor(&mut for_each.sequence_constructor)
    }

    pub(crate) fn visit_for_each_group<V: AstVisitor + ?Sized>(
        v: &mut V,
        for_each_group: &mut ast::ForEachGroup,
    ) {
        for sort in for_each_group.sort.iter_mut() {
            v.visit_sort(sort)
        }
        v.visit_sequence_constructor(&mut for_each_group.sequence_constructor)
    }

    pub(crate) fn visit_fork<V: AstVisitor + ?Sized>(v: &mut V, fork: &mut ast::Fork) {
        for fallback in fork.fallbacks.iter_mut() {
            v.visit_fallback(fallback)
        }
        match &mut fork.content {
            ast::ForkContent::SequenceFallbacks(sequence_fallbacks) => {
                for (sequence, fallbacks) in sequence_fallbacks.iter_mut() {
                    v.visit_sequence(sequence);
                    for fallback in fallbacks.iter_mut() {
                        v.visit_fallback(fallback)
                    }
                }
            }
            ast::ForkContent::ForEachGroup((for_each_group, fallbacks)) => {
                v.visit_for_each_group(for_each_group);
                for fallback in fallbacks.iter_mut() {
                    v.visit_fallback(fallback)
                }
            }
        }
    }

    pub(crate) fn visit_if<V: AstVisitor + ?Sized>(v: &mut V, if_: &mut ast::If) {
        v.visit_sequence_constructor(&mut if_.sequence_constructor)
    }

    pub(crate) fn visit_iterate<V: AstVisitor + ?Sized>(v: &mut V, iterate: &mut ast::Iterate) {
        for param in iterate.params.iter_mut() {
            v.visit_param(param)
        }
        if let Some(on_completion) = &mut iterate.on_completion {
            v.visit_on_completion(on_completion)
        }
        v.visit_sequence_constructor(&mut iterate.sequence_constructor)
    }

    pub(crate) fn visit_on_completion<V: AstVisitor + ?Sized>(
        v: &mut V,
        on_completion: &mut ast::OnCompletion,
    ) {
        v.visit_sequence_constructor(&mut on_completion.sequence_constructor)
    }

    pub(crate) fn visit_map<V: AstVisitor + ?Sized>(v: &mut V, map: &mut ast::Map) {
        v.visit_sequence_constructor(&mut map.sequence_constructor)
    }

    pub(crate) fn visit_map_entry<V: AstVisitor + ?Sized>(
        v: &mut V,
        map_entry: &mut ast::MapEntry,
    ) {
        v.visit_sequence_constructor(&mut map_entry.sequence_constructor)
    }

    pub(crate) fn visit_merge<V: AstVisitor + ?Sized>(v: &mut V, merge: &mut ast::Merge) {
        for merge_source in merge.merge_sources.iter_mut() {
            v.visit_merge_source(merge_source)
        }
        v.visit_merge_action(&mut merge.merge_action);
        for fallback in merge.fallbacks.iter_mut() {
            v.visit_fallback(fallback)
        }
    }

    pub(crate) fn visit_merge_source<V: AstVisitor + ?Sized>(
        v: &mut V,
        merge_source: &mut ast::MergeSource,
    ) {
        for merge_key in merge_source.merge_keys.iter_mut() {
            v.visit_merge_key(merge_key)
        }
    }

    pub(crate) fn visit_merge_key<V: AstVisitor + ?Sized>(
        v: &mut V,
        merge_key: &mut ast::MergeKey,
    ) {
        v.visit_sequence_constructor(&mut merge_key.sequence_constructor)
    }

    pub(crate) fn visit_merge_action<V: AstVisitor + ?Sized>(
        v: &mut V,
        merge_action: &mut ast::MergeAction,
    ) {
        v.visit_sequence_constructor(&mut merge_action.sequence_constructor)
    }

    pub(crate) fn visit_message<V: AstVisitor + ?Sized>(v: &mut V, message: &mut ast::Message) {
        v.visit_sequence_constructor(&mut message.sequence_constructor)
    }

    pub(crate) fn visit_namespace<V: AstVisitor + ?Sized>(
        v: &mut V,
        namespace: &mut ast::Namespace,
    ) {
        v.visit_sequence_constructor(&mut namespace.sequence_constructor)
    }

    pub(crate) fn visit_next_iteration<V: AstVisitor + ?Sized>(
        v: &mut V,
        next_iteration: &mut ast::NextIteration,
    ) {
        for with_param in next_iteration.with_params.iter_mut() {
            v.visit_with_param(with_param)
        }
    }

    pub(crate) fn visit_next_match<V: AstVisitor + ?Sized>(
        v: &mut V,
        next_match: &mut ast::NextMatch,
    ) {
        for item in next_match.content.iter_mut() {
            match item {
                ast::NextMatchContent::WithParam(with_param) => v.visit_with_param(with_param),
                ast::NextMatchContent::Fallback(fallback) => v.visit_fallback(fallback),
            }
        }
    }

    pub(crate) fn visit_number<V: AstVisitor + ?Sized>(_v: &mut V, _number: &mut ast::Number) {
        // no children
    }

    pub(crate) fn visit_on_empty<V: AstVisitor + ?Sized>(v: &mut V, on_empty: &mut ast::OnEmpty) {
        v.visit_sequence_constructor(&mut on_empty.sequence_constructor)
    }

    pub(crate) fn visit_on_non_empty<V: AstVisitor + ?Sized>(
        v: &mut V,
        on_non_empty: &mut ast::OnNonEmpty,
    ) {
        v.visit_sequence_constructor(&mut on_non_empty.sequence_constructor)
    }

    pub(crate) fn visit_perform_sort<V: AstVisitor + ?Sized>(
        v: &mut V,
        perform_sort: &mut ast::PerformSort,
    ) {
        for sort in perform_sort.sorts.iter_mut() {
            v.visit_sort(sort)
        }
        v.visit_sequence_constructor(&mut perform_sort.sequence_constructor)
    }

    pub(crate) fn visit_processing_instruction<V: AstVisitor + ?Sized>(
        v: &mut V,
        processing_instruction: &mut ast::ProcessingInstruction,
    ) {
        v.visit_sequence_constructor(&mut processing_instruction.sequence_constructor)
    }

    pub(crate) fn visit_result_document<V: AstVisitor + ?Sized>(
        v: &mut V,
        result_document: &mut ast::ResultDocument,
    ) {
        v.visit_sequence_constructor(&mut result_document.sequence_constructor)
    }

    pub(crate) fn visit_sequence<V: AstVisitor + ?Sized>(v: &mut V, sequence: &mut ast::Sequence) {
        v.visit_sequence_constructor(&mut sequence.sequence_constructor)
    }

    pub(crate) fn visit_source_document<V: AstVisitor + ?Sized>(
        v: &mut V,
        source_document: &mut ast::SourceDocument,
    ) {
        v.visit_sequence_constructor(&mut source_document.sequence_constructor)
    }

    pub(crate) fn visit_text<V: AstVisitor + ?Sized>(_v: &mut V, _text: &mut ast::Text) {
        // no children
    }

    pub(crate) fn visit_try<V: AstVisitor + ?Sized>(v: &mut V, try_: &mut ast::Try) {
        v.visit_sequence_constructor(&mut try_.sequence_constructor);
        v.visit_catch(&mut try_.catch);
        for catch in try_.catches.iter_mut() {
            match catch {
                ast::TryCatchOrFallback::Catch(catch) => v.visit_catch(catch),
                ast::TryCatchOrFallback::Fallback(fallback) => v.visit_fallback(fallback),
            }
        }
    }

    pub(crate) fn visit_catch<V: AstVisitor + ?Sized>(v: &mut V, catch: &mut ast::Catch) {
        v.visit_sequence_constructor(&mut catch.sequence_constructor)
    }

    pub(crate) fn visit_value_of<V: AstVisitor + ?Sized>(v: &mut V, value_of: &mut ast::ValueOf) {
        v.visit_sequence_constructor(&mut value_of.sequence_constructor)
    }

    pub(crate) fn visit_where_populated<V: AstVisitor + ?Sized>(
        v: &mut V,
        where_populated: &mut ast::WherePopulated,
    ) {
        v.visit_sequence_constructor(&mut where_populated.sequence_constructor)
    }
}
