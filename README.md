# pairwise-ranked
Web Application to help rank a set of N items by repeatedly comparing pairs

## isort

An implementation of the Ford-Johnson [merge-insertion
sort](https://en.wikipedia.org/wiki/Merge-insertion_sort), which is a sorting
algorithm that requires very few comparisons. I am not convinced it's a
completely faithful implementation --- it sorts correctly, but I think there
are circumstances where it will perform extra comparisons.

This is useful because comparisons are the most expensive part of generating a
interactive pairwise ordering, since they require human interaction.

## app

This is a [yew](https://github.com/yewstack/yew)-based web app that fetches a
list of items from a [Firebase
Database](https://firebase.google.com/docs/database) instance and then
repeatedly polls the user for pairwise orderings (i.e., "which is better, A or
B?").

Given a set of these pairwise orderings, it attempts to sort the list. In this
process, we may come across pairwise orderings that we don't have yet. If so,
we note the first one and ask the user to compare that pair.
