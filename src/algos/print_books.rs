// You are given:
// An array pages: &[i32], where pages[i] represents the number of pages in the i-th book (or document).
// Two integers: days: i32 and limit_pages: i32.
// You need to determine whether it is possible to print all the books within days days under the following constraint:
// Each day you can print contiguous books (i.e. pick a subarray segment of books not yet printed, in their original order).
// The total number of pages printed in a single day must not exceed limit_pages.
// You must schedule the printing so that all books are printed in at most days days.
// The function should return true if there exists a feasible schedule satisfying these constraints; otherwise false.

pub fn print_books(pages: &[i32], days: i32, limit_pages: i32) -> bool {
    if days <= 0 || limit_pages <= 0 {
        return false;
    }

    let mut day_count = 1;
    let mut current_sum = 0;

    for &book in pages {
        if book > limit_pages {
            return false; // a single book cannot fit in a day
        }

        if current_sum + book <= limit_pages {
            current_sum += book;
        } else {
            // need a new day
            day_count += 1;
            current_sum = book;

            if day_count > days {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_books() {
        assert_eq!(print_books(&[100, 200, 300, 400], 2, 500), false);
        assert_eq!(print_books(&[10, 20, 30], 5, 500), true);
    }
}