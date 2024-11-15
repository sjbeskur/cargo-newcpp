


pub const EXAMPLE_TEST: &str = 
r#"
#include <gtest/gtest.h>
#include "dummy.hpp"

// Test the add function of my_library
TEST(MyLibraryTest, AddTest)
{
    EXPECT_EQ(add(3, 5), 8);
    EXPECT_EQ(add(0, 0), 0);
    EXPECT_EQ(add(-3, 7), 4);
}

// Test the subtract function of my_library
TEST(MyLibraryTest, SubtractTest)
{
    EXPECT_EQ(subtract(5, 3), 2);
    EXPECT_EQ(subtract(5, 5), 0);
    EXPECT_EQ(subtract(7, -3), 10);
}
"#;