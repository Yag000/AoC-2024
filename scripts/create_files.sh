
FOLDER="src"
INPUTS="input"

function create_day() {
    DAY=$1
    mkdir -p "$FOLDER/day$DAY"
    touch "$FOLDER/day$DAY/mod.rs"

    echo "pub mod day$DAY;" >> "$FOLDER/lib.rs"

    { 
    echo "pub fn part1() -> String {" ;
    echo "    let path = \"$INPUTS/day$DAY.txt\";" ;
    echo "    let input = std::fs::read_to_string(path).unwrap();" ;
    echo "    \"\".to_string()";
    echo "}";

    echo "" ;

    echo "pub fn part2() -> String {" ;
    echo "    let path = \"$INPUTS/day$DAY.txt\";" ;
    echo "    let input = std::fs::read_to_string(path).unwrap();" ;
    echo "    \"\".to_string()";
    echo "}"; 
    } >> "$FOLDER/day$DAY/mod.rs"
}


if [ $# -eq 0 ]
then
    for i in {1..25}
    do
        create_day "$i"
    done
else
        create_day "$1"
fi


