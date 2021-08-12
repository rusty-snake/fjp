fjp's homepage
==============

Source code of <https://rusty-snake.github.io/fjp>.

local preview
-------------

**Dependencies:** [ruby](https://www.ruby-lang.org/) and [bundler](https://bundler.io/).

Fedora: `sudo dnf install ruby ruby-devel rubygem-bundler rubygem-json rubygem-bigdecimal`

```bash
git clone https://github.com/rusty-snake/fjp.git
cd fjp
git worktree add ../fjp-gh-pages gh-pages
cd ../fjp-gh-pages
bundle config set --local path 'vendor/bundle'
bundle install
bundle exec jekyll serve
```

jekyll does not work with ruby 3.0 ATOW.
For Fedora 34 see <https://blog.kagesenshi.org/2021/05/ruby24-fedora34.html>.
