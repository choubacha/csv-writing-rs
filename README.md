## Serde CSV Testing

---

I wished to know if I could have nested structs easily in the serde-csv
crate to make the output generic. And while you can, it does present an
issue with the headers. The header rules cannot derive the inner structs
field set. When I attempted to write out the csv with headers I got this
error:

```
Error(Serialize("cannot serialize Inner container inside struct when writing headers from structs"))
```

We could devise a trait that outputs the "headers" as a function and the
records as a separate function to work with the csv output.
