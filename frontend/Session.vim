let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/projects/colorful-nails/frontend
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +10 ~/projects/colorful-nails/frontend/src/components/nav_btn.rs
badd +55 ~/projects/colorful-nails/frontend/src/pages/home.rs
badd +81 ~/projects/colorful-nails/frontend/public/styles.scss
argglobal
%argdel
$argadd src/main.rs
edit ~/projects/colorful-nails/frontend/src/pages/home.rs
argglobal
balt ~/projects/colorful-nails/frontend/src/components/nav_btn.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=99
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
sil! 1,2fold
sil! 16,17fold
sil! 16,18fold
sil! 16,19fold
sil! 16,20fold
sil! 15,21fold
sil! 9,24fold
sil! 8,25fold
sil! 7,54fold
sil! 6,55fold
let &fdl = &fdl
let s:l = 55 - ((35 * winheight(0) + 19) / 38)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 55
normal! 0
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
