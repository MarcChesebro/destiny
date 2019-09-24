# destiny
A simple library for parsing and evaluating dice strings for use in tabletop gaming.

# Examples
Roll some dice using destiny::parse_dice_string:
```rust
use destiny::parse_dice_string;

println!("{}", parse_dice_string("1d4"));
println!("{}", parse_dice_string("1d6"));
println!("{}", parse_dice_string("2d6"));
println!("{}", parse_dice_string("1d8 + 3"));
println!("{}", parse_dice_string("1d6 + 2d8"));
```
Calculate distributions using destiny::DiceDistribution:
```rust
use destiny::DiceDistribution;

let dd = DiceDistribution::new("2d6");
dd.ptable();

/* this will output:
+------+--------+--------+
| Roll | #Rolls | Roll%  |
+======+========+========+
| 2    | 1      | 2.78%  |
+------+--------+--------+
| 3    | 2      | 5.56%  |
+------+--------+--------+
| 4    | 3      | 8.33%  |
+------+--------+--------+
| 5    | 4      | 11.11% |
+------+--------+--------+
| 6    | 5      | 13.89% |
+------+--------+--------+
| 7    | 6      | 16.67% |
+------+--------+--------+
| 8    | 5      | 13.89% |
+------+--------+--------+
| 9    | 4      | 11.11% |
+------+--------+--------+
| 10   | 3      | 8.33%  |
+------+--------+--------+
| 11   | 2      | 5.56%  |
+------+--------+--------+
| 12   | 1      | 2.78%  |
+------+--------+--------+
*/
```
