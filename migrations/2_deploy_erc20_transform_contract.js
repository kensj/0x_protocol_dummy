let ERC20Transform = artifacts.require("./erc20_transform.sol");

module.exports = function(deployer) {
   deployer.deploy(ERC20Transform);
};
