// Если использовать стандартный формат именование тестов MethodName_StateUnderTest_ExpectedBehavior,
// Название этих тестов могут быть очень длинными, поэтому я решил использовать более короткий вариант. (Не смотря на то что длинное название тестов показывает, что и при каких условиях идет тест
// в реальности мы сверяем `assert`, промежуточные и финальные данные, поэтому название тестов не дает полный картины проверки, как их результаты.)

// Пример :
//================(Синтаксис)================
// - fn_<имя_функции>_<t|e>_<номер_теста>
//===========================================

// 1) указываем само имя в функции в начале имени, дальше выбираем ожидаемый результат
// 2) t_x, где `t` - это что может быть положительно ПРИ СРАВНЕНИЙ, а `x` - номер теста

// 2.1 ) Пример :
//===========================
// t_x - TRUE (match)
// e_x - Err() / (SHOULD_PANIC)
//===========================

// 2.2)`t` : к примеру наша функция возращает `true` и мы ждем `true`.
// тоже самое подходит и для `false` и `false`, и даже если условие `false` не `true`.
// Все три варианта возращают `true` потому что
// мы используем СРАВНЕНИЕ, если ваше сравнение удачно, значит это `t`
// 2.3) `x` : это номер теста, вы можете указать несколько тестов которые ждут `t` | `e`,
// поэтому мы можем, просто повышать номер с тестом, главное правило,
// счетчик должен быть уникальным для каждой функций (именно то, что мы проверяем)
// и для каждого типа `t` / `e`, так мы избавлямся от дубликатов
// 3) e_x, где `e` - это что может быть положительно ПРИ ПАНИКЕ, а `x` - номер теста
// 3.1) `e` - то что возращает панику или проброс ошибки, так можно маркировать тесты
// которые `should_panic` либо те, где мы проверяем определенный тип ошибки `error == error`
// 3.2) `x` - пункт 2.3

// Допольнительные условия (не обязательно)

// 1) Желательно каждый unit тест который проверяет одну функцию вкладывать в отдельный модуль
// (само собой всё в один модуль, всё остальное `submodule`),
// это связано с тем, что на одну функцию может быть несколько тестов, и если мы будем писать
// для каждой функций свои модуль, то мы сможем легко найти тесты для определенной функции
// с повышением количество тестов. Именование модулей должно быть таким же как и у функций с префиксом `fn_`.

// 1.1) Пример :
//================(Синтаксис)================
// #[cfg(test)]
// mod tests {
//     #[cfg(test)]
//     mod fn_<имя_функции> {
//         use super::*;
//         // множество тестов для одной функций для разной обработки поведений и результатов
//     }
//     ... Повторяем для каждой функции
// }
//===========================================

// 2) если тесты пишутся для каждой ОС отдельно (к примеру для Windows и Linux),
// используйте разделение имен тестов в родительском модуле, а не в дочернем
// 3) комментируйте каким образом сравниваем, после промежуточные (если есть), финальные и просто, что сравнивается в assert, ЕСЛИ это не очевидно из самого кода

// Unit тесты
#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(test)]
    mod convert_tests {
        use super::*;

        mod fn_bytes_to_string_utf8 {
            use super::*;

            #[test]
            fn bytes_to_string_utf8_t_0() {
                assert_eq!(
                    convert::bytes_to_string_utf8("!!! 😊 😎 & 🚀".as_bytes()).unwrap(),
                    "!!! 😊 😎 & 🚀"
                );
            }

            #[test]
            #[should_panic]
            fn bytes_to_string_utf8_f_0() {
                pyo3::prepare_freethreaded_python();
                convert::bytes_to_string_utf8(b"\xF0\x90\x80").unwrap();
            }
        }

        mod fn_string_to_default_regex {
            use super::*;

            #[test]
            fn string_to_default_regex_t_0() {
                assert_eq!(
                    convert::string_to_default_regex(String::from("[0-9]+?")).to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            #[should_panic]
            fn string_to_default_regex_f_0() {
                convert::string_to_default_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }

            #[test]
            #[should_panic(
                expected = "error: look-around, including look-ahead and look-behind, is not supported"
            )]
            fn string_to_default_regex_f_1() {
                convert::string_to_default_regex(String::from(r"(\b\w+\b)(?=.+?\1)"));
            }
        }

        mod fn_string_to_fancy_regex {
            use super::*;
            #[test]
            fn string_to_fancy_regex_t_0() {
                assert_eq!(
                    convert::string_to_default_regex(String::from("[0-9]+?")).to_string(),
                    regex::Regex::new("[0-9]+?").unwrap().to_string()
                );
            }

            #[test]
            fn string_to_fancy_regex_t_1() {
                convert::string_to_fancy_regex(String::from(r"(\b\w+\b)(?=.+?\1)"));
            }

            #[test]
            #[should_panic]
            fn string_to_fancy_regex_f_0() {
                convert::string_to_fancy_regex(String::from(
                    r"\QThis is not a valid regex!@#$%^&*()_+\E",
                ));
            }
        }
    }
}