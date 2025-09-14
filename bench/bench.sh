rm -rf in/ out/
mkdir in/ out/
for i in {1..1000}; do
	cp mordant.md "in/mordant_$i.md"	
done

time ../target/release/mordant -c ./mordant.toml -o ./out ./in/*.md

