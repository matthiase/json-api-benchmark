const express = require('express')
const app = express()

app.use('/species', require('./species'))

app.use((err, req, res, next) => {
  res.status(err.status || 500)
  res.json({
    message: err.message,
    error: process.env.NODE_ENV === 'development' ? err : {}
  })
})

const port = process.env.PORT || 8080
app.listen(port, () => {
  console.log(`Server listening on port ${port}`)
})
