<template>
  <div class="top-bar">
    <h2>Meta Defender Test Page</h2>
    <button class="wallet-button" @click="linkWallet()">{{ address }}</button>
  </div>

  <div class="dashboard">
    <div class="total-capital">Total Capital:</div>
    <div class="total-coverage">Total Coverage:</div>
    <div class="fee-rate">Fee Rate:</div>
  </div>

  <div class="dapp">

    <div class="buy-cover">
      <div class="title">Buy Cover</div>

      <div class="amount-wrapper">
        <div class="input-wrapper">
          <input type="text" class="amount-input">
          <div class="token">USD</div>
        </div>
        <div class="excute">Buy</div>
      </div>

      <div class="caculate-fee">Payment:</div>

      <div class="policy">
        <div class="policy-title">My Policies:</div>
        <div class="policy-arg">
          <div>Index</div>
          <div>Coverage</div>
          <div>Until</div>
          <div>Claim</div>
        </div>

        <div class="policy-ins">
          <div v-if="(policyArr.length == 0)" class="no-policy">
            No policies yet.
          </div>
          <div v-else>
            <ul>
              <li v-for="(item, index) in policyArr" :key="index" class="policy-list">
                <div class="l1">{{ item.index }}</div>
                <div class="l2">{{ item.coverage }}</div>
                <div class="l3">{{ item.until }}</div>
                <div class="l4-claim">claim</div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>


    <div class="underwrite">
      <div class="title">Underwrite</div>

      <div class="amount-wrapper">
        <div class="input-wrapper">
          <input type="text" class="amount-input">
          <div class="token">USD</div>
        </div>
        <div class="excute deposit">Deposit</div>
      </div>
      <div class="caculate-fee no-lock">⭐️ No lock period requied</div>
      <div class="policy">
        <div class="policy-title">Liquidity And Rewards:</div>
        <div class="policy-title liquidity-manage">My Liquidity:</div>
        <div class="policy-title liquidity-manage">Pool Share:</div>
        <div class="policy-title liquidity-manage liquidity-claim">
          Withdrawable:
          <div class="l4-claim">claim</div>
        </div>
        <div class="policy-title liquidity-manage liquidity-claim">Premium Rewards:
          <div class="l4-claim">claim</div>
        </div>
        <div class="policy-title liquidity-manage liquidity-claim">Mining Rewards:
          <div class="l4-claim">claim</div>
        </div>
      </div>

    </div>
  </div>

  <br><br>
  <div class="get-number">
    <button class="wallet-button" @click="getNum()">get number</button>
    <div class="get-number-result">{{ (fqyGet) }}</div>
  </div>


</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
import { web3Enable, web3Accounts } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import metadata from './metadata.json';
import { toRaw } from 'vue';

export default {
  name: 'App',
  data() {
    return {
      address: 'connect wallet',
      currentAddress: '',
      contract: '',
      policyArr: [
        { index: 1, coverage: 500, until: 30 },
        { index: 2, coverage: 5000, until: 30 }
      ],
      fqyGet: 0
    }
  },
  methods: {

    async linkWallet() {
      await web3Enable('test');
      let allAccounts = await web3Accounts();
      console.log(allAccounts);
      if (allAccounts.length != 0) {
        this.currentAddress = allAccounts[0].address;
        let len = allAccounts[0].address.length;
        this.address =
          allAccounts[0].address.substring(0, 4) + '***' + allAccounts[0].address.substring(len - 4, len);
        localStorage.setItem('linkedAddress', this.address);
      }
    },

    async getNum() {
      let from = '5ETiX2fab44uNUNaUuY3AtQ276Kyjo9p43iVvzTN3RQnCHb2';
      let gasLimit = -1;
      let contractRaw = toRaw(this.contract);
      let result = await contractRaw.query.get(from, { gasLimit });
      console.log(result.result, 'hahah');
    }
  },

  async mounted() {
    console.log('mounted');

    let linkedAddress = localStorage.getItem('linkedAddress');

    if (linkedAddress) {
      this.address = linkedAddress;
    } else {
      this.address = 'connect wallet';
    }
    
    const address = '5HNC8bGYFNFzR66s3d26JhcGMKco5Lj72WpJ3Rx3QxqMbUv7';
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    await api.isReady;
    //console.log(api.isReady);
    //console.log((await api.rpc.chain.getHeader()).hash.toHex());
    this.contract = new ContractPromise(api, metadata, address);
    //console.log(this.contract);

  }
}
</script>

<style>
body {
  background-image: linear-gradient(rgb(190, 247, 247), rgb(175, 157, 227));
  height: 100vh;
  font-family: Thicccboi, sans-serif !important;
}

ul {
  margin: 0;
  padding: 0;
}

li {
  list-style: none;
  padding: 0;
  margin: 0;
  font-size: 15px;
}

.top-bar {
  display: flex;
  width: 80%;
  margin: 0 auto;
  justify-content: space-between;
  align-items: center;
}

.dashboard {
  width: 80%;
  margin: 0 auto;
  display: flex;
  margin-top: 20px;
  border: 1px solid rgb(114, 93, 198);
  font-size: 20px;
  justify-content: space-around;
  height: 50px;
  line-height: 50px;
  border-radius: 20px;
}

.dapp {
  width: 80%;
  margin: 0 auto;
  display: flex;
  margin-top: 20px;
  justify-content: space-around;
}

.dapp>div {
  width: 40%;
  height: 400px;
  border: 1px solid rgb(114, 93, 198);
  border-radius: 20px;
  overflow: hidden;
}

.buy-cover {
  background-image: linear-gradient(antiquewhite, rgb(208, 218, 146));
}

.underwrite {
  background-image: linear-gradient(rgb(197, 201, 247), rgb(237, 129, 152));
}

.title {
  width: 100%;
  text-align: center;
  font-size: 20px;
  margin-top: 10px;
}

.amount-wrapper {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 20px;
}

.input-wrapper {
  border: 1px solid rgb(203, 66, 126);
  border-radius: 10px;
  overflow: hidden;
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  background-color: white;
}

.amount-wrapper input {
  width: 70%;
  height: 34px;
  padding: 0;
  border: none;
}

.amount-wrapper input:focus {
  outline: none;
}

.token {
  background-color: rgb(206, 204, 208);
  width: 23%;
  height: 24px;
  line-height: 24px;
  text-align: center;
  border-radius: 10px;
}

.excute {
  margin-left: 15px;
  background-image: linear-gradient(to right, rgb(114, 93, 198), rgb(203, 66, 126));
  color: white;
  height: 36px;
  width: 45px;
  line-height: 36px;
  text-align: center;
  cursor: pointer;
  border-radius: 10px;
}

.deposit {
  width: 70px;
}

.caculate-fee {
  margin-top: 8px;
  padding-left: 126px;
  font-size: 15px;
}

.no-lock {
  padding-left: 100px;
}

.policy {
  border-top: 1px solid rgb(203, 66, 126);
  margin-top: 20px;
}

.policy-title {
  padding-left: 30px;
  margin-top: 5px;
  font-weight: 600;
}

.policy-arg {
  padding-left: 30px;
  display: flex;
  font-size: 15px;
  margin-top: 8px;
}

.policy-arg>div {
  margin-right: 50px;
}

.policy-ins {

  margin: 0 auto;
  margin-top: 8px;
  width: 90%;
}

.no-policy {
  padding-left: 20px;
  font-size: 15px;
  color: rgb(203, 66, 126);
  font-weight: 800;
}

.policy-list {
  display: flex;
  padding-left: 20px;
  position: relative;
  height: 30px;
  border-bottom: 1px solid gray;
}

.policy-list>div {
  margin-top: 5px;
  position: absolute;
}

.l1 {
  left: 20px;
}

.l2 {
  left: 95px;
}

.l3 {
  left: 212px;
}

.l4-claim {
  left: 290px;
  width: 50px;
  text-align: center;
  border-radius: 10px;
  color: white !important;
  background-image: linear-gradient(to right, rgb(114, 93, 198), rgb(203, 66, 126));
  cursor: pointer;
}

.liquidity-manage {
  font-size: 15px;
  margin-top: 8px;
  font-weight: normal;
}

.liquidity-claim {
  display: flex;
  justify-content: space-between;
  padding-right: 20px;
}

.wallet-button {
  width: 200px;
  height: 50px;
  font-size: 20px;
  border: none;
  background-image: linear-gradient(to right, rgb(114, 93, 198), rgb(203, 66, 126));
  color: white;
  border-radius: 10px;
  cursor: pointer;
}

.wallet-button:hover {
  box-shadow: 0px 0px 5px 5px rgb(212, 102, 199);
}

.get-number {
  display: flex;
}

.get-number-result {
  width: 50px;
  height: 50px;
  border: 1px solid blue;
  margin-left: 10px;
  line-height: 50px;
  text-align: center;
}

@font-face {
  font-family: 'Thicccboi';
  src: url('./style/fonts/THICCCBOI-Bold.woff2') format('woff2'), url('./style/fonts/THICCCBOI-Bold.eot') format('embedded-opentype'), url('./style/fonts/THICCCBOI-Bold.ttf') format('truetype'), url('./style/fonts/THICCCBOI-Bold.otf') format('opentype'), url('./style/fonts/THICCCBOI-Bold.svg') format('svg');
  font-weight: 700;
  font-style: normal;
  font-display: swap;
}

@font-face {
  font-family: 'Thicccboi';
  src: url('./style/fonts/THICCCBOI-Medium.woff2') format('woff2'), url('./style/fonts/THICCCBOI-Medium.eot') format('embedded-opentype'), url('./style/fonts/THICCCBOI-Medium.ttf') format('truetype'), url('./style/fonts/THICCCBOI-Medium.otf') format('opentype'), url('./style/fonts/THICCCBOI-Medium.svg') format('svg');
  font-weight: 500;
  font-style: normal;
  font-display: swap;
}

@font-face {
  font-family: 'Thicccboi';
  src: url('./style/fonts/THICCCBOI-Regular.woff2') format('woff2'), url('./style/fonts/THICCCBOI-Regular.eot') format('embedded-opentype'), url('./style/fonts/THICCCBOI-Regular.ttf') format('truetype'), url('./style/fonts/THICCCBOI-Regular.otf') format('opentype'), url('./style/fonts/THICCCBOI-Regular.svg') format('svg');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}

@font-face {
  font-family: 'Thicccboi';
  src: url('./style/fonts/THICCCBOI-SemiBold.woff2') format('woff2'), url('./style/fonts/THICCCBOI-SemiBold.eot') format('embedded-opentype'), url('./style/fonts/THICCCBOI-SemiBold.ttf') format('truetype'), url('./style/fonts/THICCCBOI-SemiBold.otf') format('opentype'), url('./style/fonts/THICCCBOI-SemiBold.svg') format('svg');
  font-weight: 600;
  font-style: normal;
  font-display: swap;
}
</style>
