typedef void (*test_func_t)(void);

typedef struct test_case
{
  const char *fixture_name;
  const char *test_name;
  test_func_t test_func;

  struct test_case *next;
} test_case_t;

void register_test(const char *fixture_name, const char *test_name, test_func_t test_func);
void run_tests();