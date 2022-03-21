const express = require('express')
const router = express.Router()
const knex = require('./knex')

const SPECIES_MAX_ROWS = 37

router.get('/', async (_, res) => {
  const randomId = Math.floor( Math.random() * SPECIES_MAX_ROWS + 1)
  const data = await knex('species').select(['id', 'name']).where('id', randomId)
  res.json(data)
})

module.exports = router
