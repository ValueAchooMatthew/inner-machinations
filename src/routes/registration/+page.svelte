<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    invoke("send_email", {address: "matthewtamerfarah@gmail.com", link: "http://google.com"})

</script>

<body class="h-screen text-orange-600 font-semibold py-10 relative">

    <div class="flex flex-wrap justify-center h-full text-center content-center">
        <div class="font-Nunito text-2xl text-center">
            To continue, you must first verify your email. <br>
            To verify your email, click on the link sent to your inbox from info.innermachinations@gmail.com
            <br>
            <a class="font-Montserrat font-normal text-gray-900 text-lg" href="/">Return to Home</a>
        </div>
    </div>

</body>