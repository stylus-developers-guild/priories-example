(lang dune 3.19)

(name priories)

(generate_opam_files true)

(source
 (github username/reponame))

(authors "Bayge <alex@fluidity.money>")

(maintainers "Bayge <alex@fluidity.money>")

(license LICENSE)

(documentation https://github.com/af-afk/monasteries)

(package
 (name priories)
 (synopsis "A Markov Chain priory generating algorithm.")
 (description "priories generates a random priory layout as ASCII art using a Markov Chain and splittable randomness using pringo.")
 (depends ocaml pringo)
 (tags
  ("markov chain" "randomness" "monasteries" priories)))
