# DORMTK (Doug's object relationship modeler toolkit)
A toolkit for describing and modeling complex relationships between data objects.

Here's the general plan of attack for starting development on this project.
- create a rust library that validates specifications for object relationships
- input specs of object models (`Foo` has one-to-many relation to `Bar`, and 2 text fields, etc)
- the library can generate "mappings" in JS and Python to enforce the specs at runtime (`Bar` linked to more than one `Foo` is an error)
- also generate SQL statements and export/import methods for Serializing and Deserializing objects between JSON, DataBase columns, etc.
