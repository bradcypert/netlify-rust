# \SubmissionApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_submission**](SubmissionApi.md#delete_submission) | **DELETE** /submissions/{submission_id} | 
[**list_form_submission**](SubmissionApi.md#list_form_submission) | **GET** /submissions/{submission_id} | 
[**list_form_submissions**](SubmissionApi.md#list_form_submissions) | **GET** /forms/{form_id}/submissions | 
[**list_site_submissions**](SubmissionApi.md#list_site_submissions) | **GET** /sites/{site_id}/submissions | 



## delete_submission

> delete_submission(submission_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_form_submission

> Vec<crate::models::Submission> list_form_submission(submission_id, query, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** |  | [required] |
**query** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Submission>**](submission.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_form_submissions

> Vec<crate::models::Submission> list_form_submissions(form_id, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Submission>**](submission.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_submissions

> Vec<crate::models::Submission> list_site_submissions(site_id, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Submission>**](submission.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

