# \DeployedBranchApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_site_deployed_branches**](DeployedBranchApi.md#list_site_deployed_branches) | **GET** /sites/{site_id}/deployed-branches | 



## list_site_deployed_branches

> Vec<crate::models::DeployedBranch> list_site_deployed_branches(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::DeployedBranch>**](deployedBranch.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

