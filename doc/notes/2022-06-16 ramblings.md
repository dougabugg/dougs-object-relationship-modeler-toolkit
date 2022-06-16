suppose we have the models

```
model: {
    name: "review",
    fields: [
        { name: "id", type: "primary key" },
        { name: "author", type: "pk(author)" },
        { name: "subject", type: "pk(subject)" },
        { name: "rating", type: "float" },
        { name: "content", type: "string" }
    ]
}

model: {
    name: "author",
    fields: [
        { name: "id", type: "primary key" },
        { name: "name", type: "string" },
    ]
}

model: {
    name: "subject",
    fields: [
        { name: "id", type: "primary key" },
        { name: "name", type: "string" },
    ]
}
```

what do we want automated for us when the models are built and tested?

I think what I want is to generate a class that helps perform a copy-on-write; it allows
reflection of all the fields, etc., and can "get" the model as a flat object. primary
keys would be instances with similar methods. these flat objects can then be "set" back
to the model, which saves any changes to the model. the actual update to the model, be
it in a database, or some other storage, would be performed in-place.

here's a question, how do we want to "tie back" the flat object to specific instance of
a model? a hidden field with a tag I guess?

optionally, when debugging, the flat object would have setters on all the fields, to
track sources of invalid input at runtime.

do we want to add custom input validation? why not? allow assigning a statement to be
evaluated when that field is changed. can be per field or apply to the entire instance.
if None or null is returned form this callback, assume the input was rejected, otherwise
continue with returned value.



suppose the javascript looks like
```js
class Review {
    fields = [
        { name: "id", type: "primary key" },
        { name: "author", type: Author, customValidate: userSuppliedExpr },
        { name: "subject", type: Subject, customValidate: userSuppliedExpr },
        { name: "rating", type: "float", customValidate: userSuppliedExpr },
        { name: "content", type: String, customValidate: userSuppliedExpr }
    ];
    constructor(pk) {
        this.primaryKey = pk;
    }
    
    get() {
        return {
            _class: this,
            author: PK(todoBackend()), // get from backend; REST API, local storage, etc
            subject: PK(todoBackend()),
            rating: todoBackend(), 
            content: todoBackend()
        }
    }
    set(flat) {
        let copy = {};
        if(flat._class != this)
            throw todoError();
        for(let i = 0; i < this.fields.length; i++) {
            let field = this.fields[i];
            let v = field.customValidate(flat[field.name]);
            if(v === null)
                throw todoError();
            copy[field.name] = v;
        }
        return todoBackend(copy);
    }
}
```

what features would we want next? automatic REST API setup? reading and writing from
local storage and SQL databases?

