function fenv
  set -l assignments (_fenv $argv | string split0)
  for idx in (seq 1 2 (count $assignments))
    set -gx "$assignments[$idx]" "$assignments[$(math $idx + 1)]"
  end
end
