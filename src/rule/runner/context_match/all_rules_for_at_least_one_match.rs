use super::super::*;

impl Rule {
    /*
    Используется `for` вместо `iterator`, так как возращаем `NextStep`, при
    использований цикла (`for`), мы можем сделать ранний выход из функции, если
    возникла ошибка
     */

    /// Проверяем, что все правила сработают хотя бы на одно совпадение (текст)
    pub fn all_rules_for_at_least_one_match(
        stack: &mut VecDeque<(&Rule, CaptureData)>,
    ) -> NextStep {
        // Создаем временный стек, в который будем складывать все правила, которые нужно обработать
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        trace!("temporary stack created");
        // Начнем проход по `stack`, `stack_temp` будет расширять `stack`
        while let Some(mut frame) = stack.pop_front() {
            trace!("received frame from stack `{}`", frame.0.as_ref());
            // Проверяем, нужно ли идти дальше
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    let texts: Vec<_> = match &frame.1.text_for_capture {
                        captures::CaptureType::Single(v) => v.iter().collect(),
                        captures::CaptureType::Multiple(v) => v.iter().collect(),
                    };
                    // Хранит ошибку, если она есть
                    let mut err: Option<HashMap<String, String>> = None;
                    // Статус, что нашли одно совпадение на которое сработали все правила
                    let mut rule_matched_for_any_text = false;
                    // Помечаем цикл, чтобы выйти из него, если условие не исполнилось
                    'skip_text: for text in texts {
                        // Если есть простые подправила, то мы их проверяем
                        if let Some(simple_rules) = &frame
                            .0
                            .content_unchecked()
                            .subrules
                            .as_ref()
                            .unwrap()
                            .simple_rules
                        {
                            // 1 Этап
                            // Получаем правила из `RegexSet`
                            for index in
                                Rule::get_selected_rules(&simple_rules.rgxst.regex_set, text)
                            {
                                trace!(
                                    "retrieved rules from `RegexSet` : `{}`",
                                    &simple_rules.all_rules[index].as_ref()
                                );
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(
                                    &simple_rules.all_rules[index],
                                    text,
                                );
                                // Провверяем это правило
                                if let NextStep::Error(value) = Self::next_or_data_for_error(
                                    &simple_rules.all_rules[index],
                                    &mut captures,
                                ) {
                                    // Сохраняем данные для ошибки, если error
                                    err = value;
                                    // Пропускаем текст
                                    continue 'skip_text;
                                }
                                // Загружаем во временный стек если успех
                                temp_stack.push_back((&simple_rules.all_rules[index], captures));
                            }

                            // 2 Этап
                            // Получаем правила, которые не попали в `RegexSet`
                            for rule in simple_rules.all_rules.iter() {
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Проверяем, что мы не обрабатывали это правило ранее
                                if !temp_stack.iter().any(|&(r, _)| r == rule) {
                                    trace!(
                                        " received rules that are not in `RegexSet` : (`{}`, `{:#?}`)",
                                        &rule.as_ref(),
                                        rule.content_unchecked().requirement
                                    );
                                    // dbg!(rule);
                                    // Сохраняем данные для ошибки, если error
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        err = value;
                                        continue 'skip_text;
                                    }
                                    // Загружаем во временный стек, если успех
                                    temp_stack.push_back((rule, captures));
                                }
                            }
                        }
                        // Если есть сложные подправила, то мы их проверяем
                        if let Some(complex_rules) = &frame
                            .0
                            .content_unchecked()
                            .subrules
                            .as_ref()
                            .unwrap()
                            .complex_rules
                        {
                            // 3 Этап
                            // Получаем сложные правила
                            for rule in complex_rules {
                                trace!(
                                    " complex rule -  : (`{}`, `{:#?}`)",
                                    &rule.as_ref(),
                                    rule.content_unchecked().requirement
                                );
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Сохраняем данные для ошибки, если error
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    err = value;
                                    continue 'skip_text;
                                }

                                temp_stack.push_back((rule, captures));
                            }
                        }

                        info!("all rules passed successfully for the text `{}` ", text);
                        // Если дошли до конца цикла (в рамках одного элемента), значит все правила сработали
                        rule_matched_for_any_text = true;
                        break;
                    }
                    if rule_matched_for_any_text {
                        // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
                        stack.extend(temp_stack.drain(..));
                    } else {
                        // ================= (LOG) =================
                        error!("all of the rules do not match any text");

                        // =========================================
                        return NextStep::Error(err);
                    }
                }
                // Завершены все действия для правила
                NextStep::Finish => (),
                // Условие не сработало, значит ошибка
                NextStep::Error(value) => {
                    // ================= (LOG) =================
                    error!("all of the rules do not match any text");

                    // =========================================
                    return NextStep::Error(value);
                }
            }
        }

        NextStep::Finish
    }
}
