module.exports = {
  development: {
    client: 'postgresql',
    connection: {
      host: 'localhost',
      database: 'starwars',
      user: 'postgres',
      password: 'password'
    },
    debug: true
  },

  production: {
    client: 'postgresql',
    connection: {
      host: 'localhost',
      database: 'starwars',
      user: 'postgres',
      password: 'password'
    },
    pool: {
      min: 1,
      max: 10
    }
  }
}
