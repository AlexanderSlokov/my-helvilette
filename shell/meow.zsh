# meow.zsh: Helvilette's blue eyes in the zsh right prompt (ADR-0004).
#
# Privacy: this plugin never reads $BUFFER or the command line. It uses only
# the fact that the line was redrawn (a keystroke), the time, and the exit
# status of the last command. It starts no process of its own.
#
# Enable it in ~/.zshrc, after your theme:  eval "$(meow init zsh)"

[[ -o interactive ]] || return 0
(( ${+_meow_loaded} )) && return 0
typeset -g _meow_loaded=1
zmodload zsh/datetime zsh/sched || return 0
autoload -Uz add-zsh-hook add-zle-hook-widget

# Must match DROWSY_AFTER and ASLEEP_AFTER in src/cat.rs.
typeset -gi _meow_drowsy_after=30 _meow_asleep_after=90
# Redraw the prompt at most this often: a redraw expands the theme's prompt again.
typeset -gF _meow_min_gap=0.15
typeset -g _meow_mood=awake _meow_last=$EPOCHREALTIME _meow_drawn=0 _meow_rprompt=
typeset -g _meow_base_rprompt=$RPROMPT
typeset -gi _meow_shift=0 _meow_busy=0

_meow_compose() {
  local eyes
  case $_meow_mood in
    awake)  eyes='o o' ;;
    blink)  eyes='- -' ;;
    grumpy) eyes='> <' ;;
    drowsy) eyes='= =' ;;
    asleep) eyes='- - z' ;;
  esac
  (( _meow_shift )) && eyes=" $eyes" || eyes="$eyes "
  # Keep the theme's own right prompt, and replace only the eyes.
  [[ $RPROMPT == "$_meow_rprompt" ]] || _meow_base_rprompt=$RPROMPT
  _meow_rprompt="${_meow_base_rprompt}%{"$'\e[38;2;159;230;255m'"%}${eyes}%{"$'\e[0m'"%}"
  RPROMPT=$_meow_rprompt
}

_meow_show() {
  _meow_compose
  _meow_busy=1
  zle reset-prompt
  _meow_busy=0
}

# Runs before each redraw of the command line, that is, on each keystroke.
_meow_redraw() {
  (( _meow_busy )) && return
  _meow_last=$EPOCHREALTIME
  local woke=0
  if [[ $_meow_mood == (drowsy|asleep|blink) ]]; then
    _meow_mood=awake
    woke=1
  fi
  (( !woke && EPOCHREALTIME - _meow_drawn < _meow_min_gap )) && return
  _meow_drawn=$EPOCHREALTIME
  (( _meow_shift = !_meow_shift ))
  _meow_show
}

_meow_precmd() {
  local st=$?
  _meow_last=$EPOCHREALTIME
  # 130 is Ctrl+C: the user stopped the command, it did not fail.
  if (( st == 0 || st == 130 )); then _meow_mood=awake; else _meow_mood=grumpy; fi
  _meow_compose
}

# A recurring timer: the cat gets drowsy, falls asleep, and sometimes slow-blinks.
_meow_tick() {
  sched +10 _meow_tick
  local idle=$(( EPOCHREALTIME - _meow_last )) mood=$_meow_mood
  if (( idle >= _meow_asleep_after )); then
    mood=asleep
  elif (( idle >= _meow_drowsy_after )); then
    mood=drowsy
  elif [[ $mood == awake ]] && (( RANDOM % 4 == 0 )); then
    mood=blink
    sched +1 _meow_unblink
  fi
  [[ $mood == $_meow_mood ]] && return
  _meow_mood=$mood
  zle && _meow_show || _meow_compose
}

_meow_unblink() {
  [[ $_meow_mood == blink ]] || return
  _meow_mood=awake
  zle && _meow_show || _meow_compose
}

zle -N _meow_redraw
add-zle-hook-widget line-pre-redraw _meow_redraw
add-zsh-hook precmd _meow_precmd
sched +10 _meow_tick
_meow_compose
