
trunk build --release
sed -i 's/href="\//href="\/demo\//' dist/index.html && sed -i "s/'\/index-/'\/demo\/index-/g" dist/index.html
rm -rf docs/ 
mv dist/ docs/
git add . 
git commit -m "fix minr bugs"
git push
