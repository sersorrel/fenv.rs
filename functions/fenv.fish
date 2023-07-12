function fenv
  set -l assignments (_fenv $argv | string split0)
  set -l n (count $assignments)
  if test $n -ne 0
    for idx in (seq 1 2 $n)
      set -gx "$assignments[$idx]" "$assignments[$(math $idx + 1)]"
    end
  end
end
