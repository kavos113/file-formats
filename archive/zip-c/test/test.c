#include "test.h"

#ifndef NULL
#define NULL 0
#endif

test_case_t *test_cases_head = NULL;
test_case_t *test_cases_tail = NULL;

void register_test(const char *fixture_name, const char *test_name, test_func_t test_func)
{
  test_case_t *new_test_case = (test_case_t *)malloc(sizeof(test_case_t));
  new_test_case->fixture_name = fixture_name;
  new_test_case->test_name = test_name;
  new_test_case->test_func = test_func;
  new_test_case->next = NULL;

  if (test_cases_head == NULL)
  {
    test_cases_head = new_test_case;
    test_cases_tail = new_test_case;
  }
  else
  {
    test_cases_tail->next = new_test_case;
    test_cases_tail = new_test_case;
  }
}

void run_tests()
{
  test_case_t *current = test_cases_head;
  while (current != NULL)
  {
    printf("Running test: %s - %s\n", current->fixture_name, current->test_name);
    current->test_func();
    current = current->next;
  }
}