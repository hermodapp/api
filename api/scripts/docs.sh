cargo d
>&2 echo "Copying documentation to static GitHub Pages site..."
cp -r target/doc/hermod ../docs/api/hermod 
# If you've added new dependencies, run cp -r target/doc ../docs/api 