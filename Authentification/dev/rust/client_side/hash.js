import init, { sha2 } from './pkg/hasher_passwd.js'

      async function run() {
        await init()

        //document.body.textContent = helloworld()
        console.log(sha2())
      }

      run()