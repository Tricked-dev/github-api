use crate::{client::Client, end_points::*, Error};
use reqwest::Body;
use serde::{Deserialize, Serialize};
impl Client {
    /// * tags meta
    /// * get `/`
    /// * docs <https://docs.github.com/rest/overview/resources-in-the-rest-api#root-endpoint>
    ///
    /// GitHub API Root
    /// Get Hypermedia links to resources accessible in GitHub's REST API
    pub async fn get_<T, V>(&self, query: Option<&T>, body: Option<V>) -> Result<GetResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::Get(), query, body).await
    }
    /// * tags apps
    /// * get `/app`
    /// * docs <https://docs.github.com/rest/reference/apps#get-the-authenticated-app>
    ///
    /// Get the authenticated app
    /// Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the "[List installations for the authenticated app](https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app)" endpoint.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_app<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetApp(), query, body).await
    }
    /// * tags apps
    /// * get `/app/hook/config`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-webhook-configuration-for-an-app>
    ///
    /// Get a webhook configuration for an app
    /// Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppHookConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppHookConfig(), query, body).await
    }
    /// * tags apps
    /// * patch `/app/hook/config`
    /// * docs <https://docs.github.com/rest/reference/apps#update-a-webhook-configuration-for-an-app>
    ///
    /// Update a webhook configuration for an app
    /// Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn patch_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchAppHookConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchAppHookConfig(), query, body).await
    }
    /// * tags apps
    /// * get `/app/hook/deliveries/{delivery_id}`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-delivery-for-an-app-webhook>
    ///
    /// Get a delivery for an app webhook
    /// Returns a delivery for the webhook configured for a GitHub App.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_app_hook_deliveries_delivery_id<T, V>(
        &self,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppHookDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppHookDeliveriesdeliveryId(delivery_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * get `/app/installations/{installation_id}`
    /// * docs <https://docs.github.com/rest/reference/apps#get-an-installation-for-the-authenticated-app>
    ///
    /// Get an installation for the authenticated app
    /// Enables an authenticated GitHub App to find an installation's information using the installation id. The installation's account type (`target_type`) will be either an organization or a user account, depending which account the repository belongs to.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_app_installations_installation_id<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppInstallationsinstallationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppInstallationsinstallationId(installation_id),
            query,
            body,
        )
        .await
    }
    /// * tags oauth-authorizations
    /// * get `/applications/grants/{grant_id}`
    /// * docs <https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-grant>
    ///
    /// Get a single grant
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    pub async fn get_applications_grants_grant_id<T, V>(
        &self,
        grant_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetApplicationsGrantsgrantIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetApplicationsGrantsgrantId(grant_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * post `/applications/{client_id}/token`
    /// * docs <https://docs.github.com/rest/reference/apps#check-a-token>
    ///
    /// Check a token
    /// OAuth applications can use a special API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) to use this endpoint, where the username is the OAuth application `client_id` and the password is its `client_secret`. Invalid tokens will return `404 NOT FOUND`.
    pub async fn post_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostApplicationsclientIdTokenResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * patch `/applications/{client_id}/token`
    /// * docs <https://docs.github.com/rest/reference/apps#reset-a-token>
    ///
    /// Reset a token
    /// OAuth applications can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
    pub async fn patch_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchApplicationsclientIdTokenResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * post `/applications/{client_id}/token/scoped`
    /// * docs <https://docs.github.com/rest/reference/apps#create-a-scoped-access-token>
    ///
    /// Create a scoped access token
    /// Use a non-scoped user-to-server OAuth access token to create a repository scoped and/or permission scoped user-to-server OAuth access token. You can specify which repositories the token can access and which permissions are granted to the token. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
    pub async fn post_applications_client_id_token_scoped<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostApplicationsclientIdTokenScopedResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdTokenScoped(client_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * get `/apps/{app_slug}`
    /// * docs <https://docs.github.com/rest/reference/apps/#get-an-app>
    ///
    /// Get an app
    /// **Note**: The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).
    ///
    /// If the GitHub App you specify is public, you can access this endpoint without authenticating. If the GitHub App you specify is private, you must authenticate with a [personal access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) or an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
    pub async fn get_apps_app_slug<T, V>(
        &self,
        app_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppsappSlugResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppsappSlug(app_slug), query, body)
            .await
    }
    /// * tags oauth-authorizations
    /// * put `/authorizations/clients/{client_id}`
    /// * docs <https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app>
    ///
    /// Get-or-create an authorization for a specific app
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
    ///
    /// Creates a new authorization for the specified OAuth application, only if an authorization for that application doesn't already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    ///
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    pub async fn put_authorizations_clients_client_id<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutAuthorizationsClientsclientIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientId(client_id),
            query,
            body,
        )
        .await
    }
    /// * tags oauth-authorizations
    /// * put `/authorizations/clients/{client_id}/{fingerprint}`
    /// * docs <https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app-and-fingerprint>
    ///
    /// Get-or-create an authorization for a specific app and fingerprint
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
    ///
    /// This method will create a new authorization for the specified OAuth application, only if an authorization for that application and fingerprint do not already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. `fingerprint` is a unique string to distinguish an authorization from others created for the same client ID and user. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    pub async fn put_authorizations_clients_client_id_fingerprint<T, V>(
        &self,
        client_id: String,
        fingerprint: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutAuthorizationsClientsclientIdfingerprintResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientIdfingerprint(client_id, fingerprint),
            query,
            body,
        )
        .await
    }
    /// * tags oauth-authorizations
    /// * get `/authorizations/{authorization_id}`
    /// * docs <https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-authorization>
    ///
    /// Get a single authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    pub async fn get_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAuthorizationsauthorizationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
    }
    /// * tags oauth-authorizations
    /// * patch `/authorizations/{authorization_id}`
    /// * docs <https://docs.github.com/rest/reference/oauth-authorizations#update-an-existing-authorization>
    ///
    /// Update an existing authorization
    /// **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
    ///
    /// If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
    ///
    /// You can only send one of these scope keys at a time.
    pub async fn patch_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchAuthorizationsauthorizationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
    }
    /// * tags codes-of-conduct
    /// * get `/codes_of_conduct/{key}`
    /// * docs <https://docs.github.com/rest/reference/codes-of-conduct#get-a-code-of-conduct>
    ///
    /// Get a code of conduct
    ///
    pub async fn get_codes_of_conduct_key<T, V>(
        &self,
        key: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetCodesOfConductkeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetCodesOfConductkey(key), query, body)
            .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-github-actions-permissions-for-an-enterprise>
    ///
    /// Get GitHub Actions permissions for an enterprise
    /// Gets the GitHub Actions permissions policy for organizations and allowed actions in an enterprise.
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_permissions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissions(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions/organizations`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-selected-organizations-enabled-for-github-actions-in-an-enterprise>
    ///
    /// List selected organizations enabled for GitHub Actions in an enterprise
    /// Lists the organizations that are selected to have GitHub Actions enabled in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_permissions_organizations<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsOrganizationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsOrganizations(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/permissions/selected-actions`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-allowed-actions-for-an-enterprise>
    ///
    /// Get allowed actions for an enterprise
    /// Gets the selected actions that are allowed in an enterprise. To use this endpoint, the enterprise permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
    ///
    /// You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_permissions_selected_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsSelectedActions(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runner-groups-for-an-enterprise>
    ///
    /// List self-hosted runner groups for an enterprise
    /// Lists all self-hosted runner groups for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runner_groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroups(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-group-for-an-enterprise>
    ///
    /// Get a self-hosted runner group for an enterprise
    /// Gets a specific self-hosted runner group for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * patch `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#update-a-self-hosted-runner-group-for-an-enterprise>
    ///
    /// Update a self-hosted runner group for an enterprise
    /// Updates the `name` and `visibility` of a self-hosted runner group in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn patch_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-organization-access-to-a-self-hosted-runner-group-in-a-enterprise>
    ///
    /// List organization access to a self-hosted runner group in an enterprise
    /// Lists the organizations with access to a self-hosted runner group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_organizations<
        T,
        V,
    >(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-in-a-group-for-an-enterprise>
    ///
    /// List self-hosted runners in a group for an enterprise
    /// Lists the self-hosted runners that are in a specific enterprise group.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-for-an-enterprise>
    ///
    /// List self-hosted runners for an enterprise
    /// Lists all self-hosted runners configured for an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runners<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunners(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners/{runner_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-for-an-enterprise>
    ///
    /// Get a self-hosted runner for an enterprise
    /// Gets a specific self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runners_runner_id<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerId(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-labels-for-a-self-hosted-runner-for-an-enterprise>
    ///
    /// List labels for a self-hosted runner for an enterprise
    /// Lists all labels for a self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn get_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * post `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#add-custom-labels-to-a-self-hosted-runner-for-an-enterprise>
    ///
    /// Add custom labels to a self-hosted runner for an enterprise
    /// Add custom labels to a self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn post_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * put `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#set-custom-labels-for-a-self-hosted-runner-for-an-enterprise>
    ///
    /// Set custom labels for a self-hosted runner for an enterprise
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in an enterprise.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn put_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#remove-all-custom-labels-from-a-self-hosted-runner-for-an-enterprise>
    ///
    /// Remove all custom labels from a self-hosted runner for an enterprise
    /// Remove all custom labels from a self-hosted runner configured in an
    /// enterprise. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(
                enterprise, runner_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * delete `/enterprises/{enterprise}/actions/runners/{runner_id}/labels/{name}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#remove-a-custom-label-from-a-self-hosted-runner-for-an-enterprise>
    ///
    /// Remove a custom label from a self-hosted runner for an enterprise
    /// Remove a custom label from a self-hosted runner configured
    /// in an enterprise. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `manage_runners:enterprise` scope to use this endpoint.
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels_name<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(
                enterprise, runner_id, name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/actions`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-enterprise>
    ///
    /// Get GitHub Actions billing for an enterprise
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// The authenticated user must be an enterprise admin.
    pub async fn get_enterprises_enterprise_settings_billing_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingActions(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/advanced-security`
    /// * docs <https://docs.github.com/rest/reference/billing#export-advanced-security-active-committers-data-for-enterprise>
    ///
    /// Get GitHub Advanced Security active committers for an enterprise
    /// Gets the GitHub Advanced Security active committers for an enterprise per repository.
    /// Each distinct user login across all repositories is counted as a single Advanced Security seat, so the total_advanced_security_committers is not the sum of active_users for each repository.
    pub async fn get_enterprises_enterprise_settings_billing_advanced_security<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingAdvancedSecurityResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/packages`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-enterprise>
    ///
    /// Get GitHub Packages billing for an enterprise
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// The authenticated user must be an enterprise admin.
    pub async fn get_enterprises_enterprise_settings_billing_packages<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingPackages(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/enterprises/{enterprise}/settings/billing/shared-storage`
    /// * docs <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-enterprise>
    ///
    /// Get shared storage billing for an enterprise
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// The authenticated user must be an enterprise admin.
    pub async fn get_enterprises_enterprise_settings_billing_shared_storage<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingSharedStorage(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags activity
    /// * get `/feeds`
    /// * docs <https://docs.github.com/rest/reference/activity#get-feeds>
    ///
    /// Get feeds
    /// GitHub provides several timeline resources in [Atom](http://en.wikipedia.org/wiki/Atom_(standard)) format. The Feeds API lists all the feeds available to the authenticated user:
    ///
    /// *   **Timeline**: The GitHub global public timeline
    /// *   **User**: The public timeline for any user, using [URI template](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia)
    /// *   **Current user public**: The public timeline for the authenticated user
    /// *   **Current user**: The private timeline for the authenticated user
    /// *   **Current user actor**: The private timeline for activity created by the authenticated user
    /// *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of.
    /// *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.
    ///
    /// **Note**: Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) since current feed URIs use the older, non revocable auth tokens.
    pub async fn get_feeds<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetFeedsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetFeeds(), query, body).await
    }
    /// * tags gists
    /// * get `/gists/{gist_id}`
    /// * docs <https://docs.github.com/rest/reference/gists#get-a-gist>
    ///
    /// Get a gist
    ///
    pub async fn get_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistId(gist_id), query, body)
            .await
    }
    /// * tags gists
    /// * patch `/gists/{gist_id}`
    /// * docs <https://docs.github.com/rest/reference/gists/#update-a-gist>
    ///
    /// Update a gist
    /// Allows you to update or delete a gist file and rename gist files. Files from the previous version of the gist that aren't explicitly changed during an edit are unchanged.
    pub async fn patch_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchGistsgistIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchGistsgistId(gist_id), query, body)
            .await
    }
    /// * tags gists
    /// * get `/gists/{gist_id}/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/gists#get-a-gist-comment>
    ///
    /// Get a gist comment
    ///
    pub async fn get_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags gists
    /// * patch `/gists/{gist_id}/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/gists#update-a-gist-comment>
    ///
    /// Update a gist comment
    ///
    pub async fn patch_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchGistsgistIdCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags gists
    /// * get `/gists/{gist_id}/{sha}`
    /// * docs <https://docs.github.com/rest/reference/gists#get-a-gist-revision>
    ///
    /// Get a gist revision
    ///
    pub async fn get_gists_gist_id_sha<T, V>(
        &self,
        gist_id: String,
        sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdshaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistIdsha(gist_id, sha), query, body)
            .await
    }
    /// * tags gitignore
    /// * get `/gitignore/templates/{name}`
    /// * docs <https://docs.github.com/rest/reference/gitignore#get-a-gitignore-template>
    ///
    /// Get a gitignore template
    /// The API also allows fetching the source of a single template.
    /// Use the raw [media type](https://docs.github.com/rest/overview/media-types/) to get the raw contents.
    pub async fn get_gitignore_templates_name<T, V>(
        &self,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGitignoreTemplatesnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGitignoreTemplatesname(name), query, body)
            .await
    }
    /// * tags apps
    /// * get `/installation/repositories`
    /// * docs <https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-app-installation>
    ///
    /// List repositories accessible to the app installation
    /// List repositories that an app installation can access.
    ///
    /// You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
    pub async fn get_installation_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetInstallationRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetInstallationRepositories(), query, body)
            .await
    }
    /// * tags licenses
    /// * get `/licenses/{license}`
    /// * docs <https://docs.github.com/rest/reference/licenses#get-a-license>
    ///
    /// Get a license
    ///
    pub async fn get_licenses_license<T, V>(
        &self,
        license: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetLicenseslicenseResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetLicenseslicense(license), query, body)
            .await
    }
    /// * tags apps
    /// * get `/marketplace_listing/accounts/{account_id}`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account>
    ///
    /// Get a subscription plan for an account
    /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    pub async fn get_marketplace_listing_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMarketplaceListingAccountsaccountIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingAccountsaccountId(account_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * get `/marketplace_listing/stubbed/accounts/{account_id}`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account-stubbed>
    ///
    /// Get a subscription plan for an account (stubbed)
    /// Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
    ///
    /// GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
    pub async fn get_marketplace_listing_stubbed_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMarketplaceListingStubbedAccountsaccountIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingStubbedAccountsaccountId(account_id),
            query,
            body,
        )
        .await
    }
    /// * tags meta
    /// * get `/meta`
    /// * docs <https://docs.github.com/rest/reference/meta#get-github-meta-information>
    ///
    /// Get GitHub meta information
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://help.github.com/articles/about-github-s-ip-addresses/)."
    ///
    /// **Note:** The IP addresses shown in the documentation's response are only example values. You must always query the API directly to get the latest list of IP addresses.
    pub async fn get_meta<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMetaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetMeta(), query, body).await
    }
    /// * tags activity
    /// * get `/notifications/threads/{thread_id}`
    /// * docs <https://docs.github.com/rest/reference/activity#get-a-thread>
    ///
    /// Get a thread
    ///
    pub async fn get_notifications_threads_thread_id<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetNotificationsThreadsthreadIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadId(thread_id),
            query,
            body,
        )
        .await
    }
    /// * tags activity
    /// * get `/notifications/threads/{thread_id}/subscription`
    /// * docs <https://docs.github.com/rest/reference/activity#get-a-thread-subscription-for-the-authenticated-user>
    ///
    /// Get a thread subscription for the authenticated user
    /// This checks to see if the current user is subscribed to a thread. You can also [get a repository subscription](https://docs.github.com/rest/reference/activity#get-a-repository-subscription).
    ///
    /// Note that subscriptions are only generated if a user is participating in a conversation--for example, they've replied to the thread, were **@mentioned**, or manually subscribe to a thread.
    pub async fn get_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetNotificationsThreadsthreadIdSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
    }
    /// * tags activity
    /// * put `/notifications/threads/{thread_id}/subscription`
    /// * docs <https://docs.github.com/rest/reference/activity#set-a-thread-subscription>
    ///
    /// Set a thread subscription
    /// If you are watching a repository, you receive notifications for all threads by default. Use this endpoint to ignore future notifications for threads until you comment on the thread or get an **@mention**.
    ///
    /// You can also use this endpoint to subscribe to threads that you are currently not receiving notifications for or to subscribed to threads that you have previously ignored.
    ///
    /// Unsubscribing from a conversation in a repository that you are not watching is functionally equivalent to the [Delete a thread subscription](https://docs.github.com/rest/reference/activity#delete-a-thread-subscription) endpoint.
    pub async fn put_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutNotificationsThreadsthreadIdSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * get `/organizations/{organization_id}/custom_roles`
    /// * docs <https://docs.github.com/rest/reference/orgs#list-custom-repository-roles-in-an-organization>
    ///
    /// List custom repository roles in an organization
    /// List the custom repository roles available in this organization. In order to see custom
    /// repository roles in an organization, the authenticated user must be an organization owner.
    ///
    /// For more information on custom repository roles, see "[Managing custom repository roles for an organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-custom-repository-roles-for-an-organization)".
    pub async fn get_organizations_organization_id_custom_roles<T, V>(
        &self,
        organization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrganizationsorganizationIdCustomRolesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrganizationsorganizationIdCustomRoles(organization_id),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-an-organization>
    ///
    /// Get an organization
    /// To see many of the organization response values, you need to be an authenticated organization owner with the `admin:org` scope. When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://help.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).
    ///
    /// GitHub Apps with the `Organization plan` permission can use this endpoint to retrieve information about an organization's GitHub plan. See "[Authenticating with GitHub Apps](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/)" for details. For an example response, see 'Response with GitHub plan information' below."
    pub async fn get_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorg(org), query, body).await
    }
    /// * tags orgs
    /// * patch `/orgs/{org}`
    /// * docs <https://docs.github.com/rest/reference/orgs/#update-an-organization>
    ///
    /// Update an organization
    /// **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
    ///
    /// Enables an authenticated organization owner with the `admin:org` scope to update the organization's profile and member privileges.
    pub async fn patch_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchOrgsorg(org), query, body).await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions`
    /// * docs <https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-an-organization>
    ///
    /// Get GitHub Actions permissions for an organization
    /// Gets the GitHub Actions permissions policy for repositories and allowed actions in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    pub async fn get_orgs_org_actions_permissions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsPermissions(org), query, body)
            .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions/repositories`
    /// * docs <https://docs.github.com/rest/reference/actions#list-selected-repositories-enabled-for-github-actions-in-an-organization>
    ///
    /// List selected repositories enabled for GitHub Actions in an organization
    /// Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    pub async fn get_orgs_org_actions_permissions_repositories<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsRepositories(org),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/permissions/selected-actions`
    /// * docs <https://docs.github.com/rest/reference/actions#get-allowed-actions-for-an-organization>
    ///
    /// Get allowed actions for an organization
    /// Gets the selected actions that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).""
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
    pub async fn get_orgs_org_actions_permissions_selected_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsSelectedActions(org),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups`
    /// * docs <https://docs.github.com/rest/reference/actions#list-self-hosted-runner-groups-for-an-organization>
    ///
    /// List self-hosted runner groups for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists all self-hosted runner groups configured in an organization and inherited from an enterprise.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runner_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunnerGroups(org), query, body)
            .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-group-for-an-organization>
    ///
    /// Get a self-hosted runner group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Gets a specific self-hosted runner group for an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * patch `/orgs/{org}/actions/runner-groups/{runner_group_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#update-a-self-hosted-runner-group-for-an-organization>
    ///
    /// Update a self-hosted runner group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Updates the `name` and `visibility` of a self-hosted runner group in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn patch_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories`
    /// * docs <https://docs.github.com/rest/reference/actions#list-repository-access-to-a-self-hosted-runner-group-in-an-organization>
    ///
    /// List repository access to a self-hosted runner group in an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud and GitHub Enterprise Server. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists the repositories with access to a self-hosted runner group configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_repositories<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners`
    /// * docs <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-in-a-group-for-an-organization>
    ///
    /// List self-hosted runners in a group for an organization
    /// The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
    ///
    /// Lists self-hosted runners that are in a specific organization group.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners`
    /// * docs <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-an-organization>
    ///
    /// List self-hosted runners for an organization
    /// Lists all self-hosted runners configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runners<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunners(org), query, body)
            .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners/{runner_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-an-organization>
    ///
    /// Get a self-hosted runner for an organization
    /// Gets a specific self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runners_runner_id<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerId(org, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#list-labels-for-a-self-hosted-runner-for-an-organization>
    ///
    /// List labels for a self-hosted runner for an organization
    /// Lists all labels for a self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn get_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * post `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#add-custom-labels-to-a-self-hosted-runner-for-an-organization>
    ///
    /// Add custom labels to a self-hosted runner for an organization
    /// Add custom labels to a self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn post_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * put `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#set-custom-labels-for-a-self-hosted-runner-for-an-organization>
    ///
    /// Set custom labels for a self-hosted runner for an organization
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn put_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization>
    ///
    /// Remove all custom labels from a self-hosted runner for an organization
    /// Remove all custom labels from a self-hosted runner configured in an
    /// organization. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn delete_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * delete `/orgs/{org}/actions/runners/{runner_id}/labels/{name}`
    /// * docs <https://docs.github.com/rest/reference/actions#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization>
    ///
    /// Remove a custom label from a self-hosted runner for an organization
    /// Remove a custom label from a self-hosted runner configured
    /// in an organization. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `admin:org` scope to use this endpoint.
    pub async fn delete_orgs_org_actions_runners_runner_id_labels_name<T, V>(
        &self,
        org: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteOrgsorgActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabelsname(org, runner_id, name),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets`
    /// * docs <https://docs.github.com/rest/reference/actions#list-organization-secrets>
    ///
    /// List organization secrets
    /// Lists all secrets available in an organization without revealing their encrypted values. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    pub async fn get_orgs_org_actions_secrets<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsSecrets(org), query, body)
            .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/public-key`
    /// * docs <https://docs.github.com/rest/reference/actions#get-an-organization-public-key>
    ///
    /// Get an organization public key
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    pub async fn get_orgs_org_actions_secrets_public_key<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretsPublicKey(org),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/{secret_name}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-an-organization-secret>
    ///
    /// Get an organization secret
    /// Gets a single organization secret without revealing its encrypted value. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    pub async fn get_orgs_org_actions_secrets_secret_name<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretName(org, secret_name),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/orgs/{org}/actions/secrets/{secret_name}/repositories`
    /// * docs <https://docs.github.com/rest/reference/actions#list-selected-repositories-for-an-organization-secret>
    ///
    /// List selected repositories for an organization secret
    /// Lists all repositories that have been selected when the `visibility` for repository access to a secret is set to `selected`. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
    pub async fn get_orgs_org_actions_secrets_secret_name_repositories<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretssecretNameRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretNameRepositories(org, secret_name),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/external-group/{group_id}`
    /// * docs <https://docs.github.com/rest/reference/teams#external-idp-group-info-for-an-organization>
    ///
    /// Get an external group
    /// Displays information about the specific group's usage.  Provides a list of the group's external members as well as a list of teams that this group is connected to.
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    pub async fn get_orgs_org_external_group_group_id<T, V>(
        &self,
        org: String,
        group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgExternalGroupgroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgExternalGroupgroupId(org, group_id),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/external-groups`
    /// * docs <https://docs.github.com/rest/reference/teams#list-external-idp-groups-for-an-organization>
    ///
    /// List external groups in an organization
    /// Lists external groups available in an organization. You can query the groups using the `display_name` parameter, only groups with a `group_name` containing the text provided in the `display_name` parameter will be returned.  You can also limit your page results using the `per_page` parameter. GitHub generates a url-encoded `page` token using a cursor value for where the next page begins. For more information on cursor pagination, see "[Offset and Cursor Pagination explained](https://dev.to/jackmarchant/offset-and-cursor-pagination-explained-b89)."
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    pub async fn get_orgs_org_external_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgExternalGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgExternalGroups(org), query, body)
            .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-an-organization-webhook>
    ///
    /// Get an organization webhook
    /// Returns a webhook configured in an organization. To get only the webhook `config` properties, see "[Get a webhook configuration for an organization](/rest/reference/orgs#get-a-webhook-configuration-for-an-organization)."
    pub async fn get_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgHookshookId(org, hook_id), query, body)
            .await
    }
    /// * tags orgs
    /// * patch `/orgs/{org}/hooks/{hook_id}`
    /// * docs <https://docs.github.com/rest/reference/orgs#update-an-organization-webhook>
    ///
    /// Update an organization webhook
    /// Updates a webhook configured in an organization. When you update a webhook, the `secret` will be overwritten. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for an organization](/rest/reference/orgs#update-a-webhook-configuration-for-an-organization)."
    pub async fn patch_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookId(org, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}/config`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-a-webhook-configuration-for-an-organization>
    ///
    /// Get a webhook configuration for an organization
    /// Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use "[Get an organization webhook ](/rest/reference/orgs#get-an-organization-webhook)."
    ///
    /// Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:read` permission.
    pub async fn get_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * patch `/orgs/{org}/hooks/{hook_id}/config`
    /// * docs <https://docs.github.com/rest/reference/orgs#update-a-webhook-configuration-for-an-organization>
    ///
    /// Update a webhook configuration for an organization
    /// Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use "[Update an organization webhook ](/rest/reference/orgs#update-an-organization-webhook)."
    ///
    /// Access tokens must have the `admin:org_hook` scope, and GitHub Apps must have the `organization_hooks:write` permission.
    pub async fn patch_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-a-webhook-delivery-for-an-organization-webhook>
    ///
    /// Get a webhook delivery for an organization webhook
    /// Returns a delivery for a webhook configured in an organization.
    pub async fn get_orgs_org_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdDeliveriesdeliveryId(org, hook_id, delivery_id),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * get `/orgs/{org}/installation`
    /// * docs <https://docs.github.com/rest/reference/apps#get-an-organization-installation-for-the-authenticated-app>
    ///
    /// Get an organization installation for the authenticated app
    /// Enables an authenticated GitHub App to find the organization's installation information.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_orgs_org_installation<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallation(org), query, body)
            .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}/installations`
    /// * docs <https://docs.github.com/rest/reference/orgs#list-app-installations-for-an-organization>
    ///
    /// List app installations for an organization
    /// Lists all GitHub Apps in an organization. The installation count includes all GitHub Apps installed on repositories in the organization. You must be an organization owner with `admin:read` scope to use this endpoint.
    pub async fn get_orgs_org_installations<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgInstallationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallations(org), query, body)
            .await
    }
    /// * tags interactions
    /// * put `/orgs/{org}/interaction-limits`
    /// * docs <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-an-organization>
    ///
    /// Set interaction restrictions for an organization
    /// Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.
    pub async fn put_orgs_org_interaction_limits<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutOrgsorgInteractionLimits(org), query, body)
            .await
    }
    /// * tags orgs
    /// * get `/orgs/{org}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user>
    ///
    /// Get organization membership for a user
    /// In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.
    pub async fn get_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
    }
    /// * tags orgs
    /// * put `/orgs/{org}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/orgs#set-organization-membership-for-a-user>
    ///
    /// Set organization membership for a user
    /// Only authenticated organization owners can add a member to the organization or update the member's role.
    ///
    /// *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/reference/orgs#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.
    ///     
    /// *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.
    ///
    /// **Rate limits**
    ///
    /// To prevent abuse, the authenticated user is limited to 50 organization invitations per 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.
    pub async fn put_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
    }
    /// * tags migrations
    /// * get `/orgs/{org}/migrations/{migration_id}`
    /// * docs <https://docs.github.com/rest/reference/migrations#get-an-organization-migration-status>
    ///
    /// Get an organization migration status
    /// Fetches the status of a migration.
    ///
    /// The `state` of a migration can be one of the following values:
    ///
    /// *   `pending`, which means the migration hasn't started yet.
    /// *   `exporting`, which means the migration is in progress.
    /// *   `exported`, which means the migration finished successfully.
    /// *   `failed`, which means the migration failed.
    pub async fn get_orgs_org_migrations_migration_id<T, V>(
        &self,
        org: String,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgMigrationsmigrationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMigrationsmigrationId(org, migration_id),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/orgs/{org}/packages/{package_type}/{package_name}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-for-an-organization>
    ///
    /// Get a package for an organization
    /// Gets a specific package in an organization.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_orgs_org_packages_package_type_package_name<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgPackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageName(org, package_type, package_name),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-version-for-an-organization>
    ///
    /// Get a package version for an organization
    /// Gets a specific package version in an organization.
    ///
    /// You must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_orgs_org_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(
                org,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/actions`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-an-organization>
    ///
    /// Get GitHub Actions billing for an organization
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    pub async fn get_orgs_org_settings_billing_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingActions(org),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/advanced-security`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-advanced-security-active-committers-for-an-organization>
    ///
    /// Get GitHub Advanced Security active committers for an organization
    /// Gets the GitHub Advanced Security active committers for an organization per repository.
    /// Each distinct user login across all repositories is counted as a single Advanced Security seat, so the total_advanced_security_committers is not the sum of advanced_security_committers for each repository.
    /// If this organization defers to an enterprise for billing, the total_advanced_security_committers returned from the organization API may include some users that are in more than one organization, so they will only consume a single Advanced Security seat at the enterprise level.
    pub async fn get_orgs_org_settings_billing_advanced_security<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingAdvancedSecurityResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingAdvancedSecurity(org),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/packages`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-an-organization>
    ///
    /// Get GitHub Packages billing for an organization
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    pub async fn get_orgs_org_settings_billing_packages<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingPackages(org),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/orgs/{org}/settings/billing/shared-storage`
    /// * docs <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-an-organization>
    ///
    /// Get shared storage billing for an organization
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `repo` or `admin:org` scope.
    pub async fn get_orgs_org_settings_billing_shared_storage<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingSharedStorage(org),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/team-sync/groups`
    /// * docs <https://docs.github.com/rest/reference/teams#list-idp-groups-for-an-organization>
    ///
    /// List IdP groups for an organization
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups available in an organization. You can limit your page results using the `per_page` parameter. GitHub generates a url-encoded `page` token using a cursor value for where the next page begins. For more information on cursor pagination, see "[Offset and Cursor Pagination explained](https://dev.to/jackmarchant/offset-and-cursor-pagination-explained-b89)."
    pub async fn get_orgs_org_team_sync_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamSyncGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgTeamSyncGroups(org), query, body)
            .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-a-team-by-name>
    ///
    /// Get a team by name
    /// Gets a team using the team's `slug`. GitHub generates the `slug` from the team `name`.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.
    pub async fn get_orgs_org_teams_team_slug<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlug(org, team_slug),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-a-discussion>
    ///
    /// Get a discussion
    /// Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#update-a-discussion>
    ///
    /// Update a discussion
    /// Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-a-discussion-comment>
    ///
    /// Get a discussion comment
    /// Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#update-a-discussion-comment>
    ///
    /// Update a discussion comment
    /// Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion-comment>
    ///
    /// Create reaction for a team discussion comment
    /// Create a reaction to a [team discussion comment](https://docs.github.com/rest/reference/teams#discussion-comments). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number_reactions<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(org, team_slug, discussion_number, comment_number), query, body).await
    }
    /// * tags reactions
    /// * post `/orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion>
    ///
    /// Create reaction for a team discussion
    /// Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_reactions<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/external-groups`
    /// * docs <https://docs.github.com/rest/reference/teams#link-external-idp-group-team-connection>
    ///
    /// Update the connection between an external group and a team
    /// Creates a connection between a team and an external group.  Only one external group can be linked to a team.
    ///
    /// You can manage team membership with your identity provider using Enterprise Managed Users for GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)" in the GitHub Help documentation.
    pub async fn patch_orgs_org_teams_team_slug_external_groups<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugExternalGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugExternalGroups(org, team_slug),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user>
    ///
    /// Get team membership for a user
    /// Team members will include the members of child teams.
    ///
    /// To get a user's membership with a team, the team must be visible to the authenticated user.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.
    ///
    /// **Note:**
    /// The response contains the `state` of the membership and the member's `role`.
    ///
    /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
    pub async fn get_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * put `/orgs/{org}/teams/{team_slug}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user>
    ///
    /// Add or update team membership for a user
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the "pending" state until the person accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team.
    ///
    /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.
    pub async fn put_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgTeamsteamSlugMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/projects/{project_id}`
    /// * docs <https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project>
    ///
    /// Check team permissions for a project
    /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects/{project_id}`.
    pub async fn get_orgs_org_teams_team_slug_projects_project_id<T, V>(
        &self,
        org: String,
        team_slug: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugProjectsprojectId(org, team_slug, project_id),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/repos/{owner}/{repo}`
    /// * docs <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository>
    ///
    /// Check team permissions for a repository
    /// Checks whether a team has `admin`, `push`, `maintain`, `triage`, or `pull` permission for a repository. Repositories inherited through a parent team will also be checked.
    ///
    /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `application/vnd.github.v3.repository+json` accept header.
    ///
    /// If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.
    pub async fn get_orgs_org_teams_team_slug_repos_owner_repo<T, V>(
        &self,
        org: String,
        team_slug: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugReposownerrepo(org, team_slug, owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings`
    /// * docs <https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team>
    ///
    /// List IdP groups for a team
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups connected to a team on GitHub.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
    pub async fn get_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/orgs/{org}/teams/{team_slug}/team-sync/group-mappings`
    /// * docs <https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections>
    ///
    /// Create or update IdP group connections
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
    ///
    /// **Note:** You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/team-sync/group-mappings`.
    pub async fn patch_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
    }
    /// * tags projects
    /// * get `/projects/columns/cards/{card_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#get-a-project-card>
    ///
    /// Get a project card
    ///
    pub async fn get_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsColumnsCardscardIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
    }
    /// * tags projects
    /// * patch `/projects/columns/cards/{card_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#update-a-project-card>
    ///
    /// Update an existing project card
    ///
    pub async fn patch_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsColumnsCardscardIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
    }
    /// * tags projects
    /// * get `/projects/columns/{column_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#get-a-project-column>
    ///
    /// Get a project column
    ///
    pub async fn get_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsColumnscolumnIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
    }
    /// * tags projects
    /// * patch `/projects/columns/{column_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#update-a-project-column>
    ///
    /// Update an existing project column
    ///
    pub async fn patch_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsColumnscolumnIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
    }
    /// * tags projects
    /// * get `/projects/{project_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#get-a-project>
    ///
    /// Get a project
    /// Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    pub async fn get_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetProjectsprojectId(project_id), query, body)
            .await
    }
    /// * tags projects
    /// * patch `/projects/{project_id}`
    /// * docs <https://docs.github.com/rest/reference/projects#update-a-project>
    ///
    /// Update a project
    /// Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.
    pub async fn patch_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchProjectsprojectId(project_id), query, body)
            .await
    }
    /// * tags projects
    /// * get `/projects/{project_id}/collaborators/{username}/permission`
    /// * docs <https://docs.github.com/rest/reference/projects#get-project-permission-for-a-user>
    ///
    /// Get project permission for a user
    /// Returns the collaborator's permission level for an organization project. Possible values for the `permission` key: `admin`, `write`, `read`, `none`. You must be an organization owner or a project `admin` to review a user's permission level.
    pub async fn get_projects_project_id_collaborators_username_permission<T, V>(
        &self,
        project_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsprojectIdCollaboratorsusernamePermissionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsprojectIdCollaboratorsusernamePermission(project_id, username),
            query,
            body,
        )
        .await
    }
    /// * tags rate-limit
    /// * get `/rate_limit`
    /// * docs <https://docs.github.com/rest/reference/rate-limit#get-rate-limit-status-for-the-authenticated-user>
    ///
    /// Get rate limit status for the authenticated user
    /// **Note:** Accessing this endpoint does not count against your REST API rate limit.
    ///
    /// **Note:** The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    pub async fn get_rate_limit<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRateLimitResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetRateLimit(), query, body).await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-repository>
    ///
    /// Get a repository
    /// The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.
    pub async fn get_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepo(owner, repo), query, body)
            .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}`
    /// * docs <https://docs.github.com/rest/reference/repos/#update-a-repository>
    ///
    /// Update a repository
    /// **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/reference/repos#replace-all-repository-topics) endpoint.
    pub async fn patch_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchReposownerrepo(owner, repo), query, body)
            .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/artifacts`
    /// * docs <https://docs.github.com/rest/reference/actions#list-artifacts-for-a-repository>
    ///
    /// List artifacts for a repository
    /// Lists all artifacts for a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsArtifactsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifacts(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-an-artifact>
    ///
    /// Get an artifact
    /// Gets a specific artifact for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_artifacts_artifact_id<T, V>(
        &self,
        owner: String,
        repo: String,
        artifact_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsArtifactsartifactIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifactsartifactId(owner, repo, artifact_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/jobs/{job_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-job-for-a-workflow-run>
    ///
    /// Get a job for a workflow run
    /// Gets a specific job in a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_jobs_job_id<T, V>(
        &self,
        owner: String,
        repo: String,
        job_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsJobsjobIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsJobsjobId(owner, repo, job_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/permissions`
    /// * docs <https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-a-repository>
    ///
    /// Get GitHub Actions permissions for a repository
    /// Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions allowed to run in the repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    pub async fn get_repos_owner_repo_actions_permissions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissions(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/permissions/selected-actions`
    /// * docs <https://docs.github.com/rest/reference/actions#get-allowed-actions-for-a-repository>
    ///
    /// Get allowed actions for a repository
    /// Gets the settings for selected actions that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
    ///
    /// You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
    pub async fn get_repos_owner_repo_actions_permissions_selected_actions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissionsSelectedActions(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners`
    /// * docs <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-a-repository>
    ///
    /// List self-hosted runners for a repository
    /// Lists all self-hosted runners configured in a repository. You must authenticate using an access token with the `repo` scope to use this endpoint.
    pub async fn get_repos_owner_repo_actions_runners<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunners(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners/{runner_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-a-repository>
    ///
    /// Get a self-hosted runner for a repository
    /// Gets a specific self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn get_repos_owner_repo_actions_runners_runner_id<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerId(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#list-labels-for-a-self-hosted-runner-for-a-repository>
    ///
    /// List labels for a self-hosted runner for a repository
    /// Lists all labels for a self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn get_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * post `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#add-custom-labels-to-a-self-hosted-runner-for-a-repository>
    ///
    /// Add custom labels to a self-hosted runner for a repository
    /// Add custom labels to a self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn post_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * put `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#set-custom-labels-for-a-self-hosted-runner-for-a-repository>
    ///
    /// Set custom labels for a self-hosted runner for a repository
    /// Remove all previous custom labels and set the new custom labels for a specific
    /// self-hosted runner configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn put_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels`
    /// * docs <https://docs.github.com/rest/reference/actions#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository>
    ///
    /// Remove all custom labels from a self-hosted runner for a repository
    /// Remove all custom labels from a self-hosted runner configured in a
    /// repository. Returns the remaining read-only labels from the runner.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * delete `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}`
    /// * docs <https://docs.github.com/rest/reference/actions#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository>
    ///
    /// Remove a custom label from a self-hosted runner for a repository
    /// Remove a custom label from a self-hosted runner configured
    /// in a repository. Returns the remaining labels from the runner.
    ///
    /// This endpoint returns a `404 Not Found` status if the custom label is not
    /// present on the runner.
    ///
    /// You must authenticate using an access token with the `repo` scope to use this
    /// endpoint.
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(
                owner, repo, runner_id, name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs`
    /// * docs <https://docs.github.com/rest/reference/actions#list-workflow-runs-for-a-repository>
    ///
    /// List workflow runs for a repository
    /// Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRuns(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-workflow-run>
    ///
    /// Get a workflow run
    /// Gets a specific workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_runs_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunId(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts`
    /// * docs <https://docs.github.com/rest/reference/actions#list-workflow-run-artifacts>
    ///
    /// List workflow run artifacts
    /// Lists artifacts for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_runs_run_id_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdArtifactsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdArtifacts(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-workflow-run-attempt>
    ///
    /// Get a workflow run attempt
    /// Gets a specific workflow run attempt. Anyone with read access to the repository
    /// can use this endpoint. If the repository is private you must use an access token
    /// with the `repo` scope. GitHub Apps must have the `actions:read` permission to
    /// use this endpoint.
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs`
    /// * docs <https://docs.github.com/rest/reference/actions#list-jobs-for-a-workflow-run-attempt>
    ///
    /// List jobs for a workflow run attempt
    /// Lists jobs for a specific workflow run attempt. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/jobs`
    /// * docs <https://docs.github.com/rest/reference/actions#list-jobs-for-a-workflow-run>
    ///
    /// List jobs for a workflow run
    /// Lists jobs for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    pub async fn get_repos_owner_repo_actions_runs_run_id_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdJobsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdJobs(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/runs/{run_id}/timing`
    /// * docs <https://docs.github.com/rest/reference/actions#get-workflow-run-usage>
    ///
    /// Get workflow run usage
    /// Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_runs_run_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdTimingResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdTiming(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets`
    /// * docs <https://docs.github.com/rest/reference/actions#list-repository-secrets>
    ///
    /// List repository secrets
    /// Lists all secrets available in a repository without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_secrets<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecrets(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets/public-key`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-repository-public-key>
    ///
    /// Get a repository public key
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_secrets_public_key<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretsPublicKey(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/secrets/{secret_name}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-repository-secret>
    ///
    /// Get a repository secret
    /// Gets a single repository secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_secrets_secret_name<T, V>(
        &self,
        owner: String,
        repo: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretssecretName(owner, repo, secret_name),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows`
    /// * docs <https://docs.github.com/rest/reference/actions#list-repository-workflows>
    ///
    /// List repository workflows
    /// Lists the workflows in a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_workflows<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflows(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-a-workflow>
    ///
    /// Get a workflow
    /// Gets a specific workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowId(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs`
    /// * docs <https://docs.github.com/rest/reference/actions#list-workflow-runs>
    ///
    /// List workflow runs
    /// List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdRuns(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing`
    /// * docs <https://docs.github.com/rest/reference/actions#get-workflow-usage>
    ///
    /// Get workflow usage
    /// Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdTimingResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdTiming(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/autolinks/{autolink_id}`
    /// * docs <https://docs.github.com/v3/repos#get-autolink>
    ///
    /// Get an autolink reference of a repository
    /// This returns a single autolink reference by ID that was configured for the given repository.
    ///
    /// Information about autolinks are only available to repository administrators.
    pub async fn get_repos_owner_repo_autolinks_autolink_id<T, V>(
        &self,
        owner: String,
        repo: String,
        autolink_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoAutolinksautolinkIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoAutolinksautolinkId(owner, repo, autolink_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-branch>
    ///
    /// Get a branch
    ///
    pub async fn get_repos_owner_repo_branches_branch<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranch(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection`
    /// * docs <https://docs.github.com/rest/reference/repos#get-branch-protection>
    ///
    /// Get branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    pub async fn get_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/branches/{branch}/protection`
    /// * docs <https://docs.github.com/rest/reference/repos#update-branch-protection>
    ///
    /// Update branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Protecting a branch requires admin or owner permissions to the repository.
    ///
    /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
    ///
    /// **Note**: The list of users, apps, and teams in total is limited to 100 items.
    pub async fn put_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoBranchesbranchProtectionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins`
    /// * docs <https://docs.github.com/rest/reference/repos#get-admin-branch-protection>
    ///
    /// Get admin branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    pub async fn get_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionEnforceAdminsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins`
    /// * docs <https://docs.github.com/rest/reference/repos#set-admin-branch-protection>
    ///
    /// Set admin branch protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
    pub async fn post_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoBranchesbranchProtectionEnforceAdminsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews`
    /// * docs <https://docs.github.com/rest/reference/repos#get-pull-request-review-protection>
    ///
    /// Get pull request review protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    pub async fn get_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews`
    /// * docs <https://docs.github.com/rest/reference/repos#update-pull-request-review-protection>
    ///
    /// Update pull request review protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
    ///
    /// **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures`
    /// * docs <https://docs.github.com/rest/reference/repos#get-commit-signature-protection>
    ///
    /// Get commit signature protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://help.github.com/articles/signing-commits-with-gpg) in GitHub Help.
    ///
    /// **Note**: You must enable branch protection to require signed commits.
    pub async fn get_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures`
    /// * docs <https://docs.github.com/rest/reference/repos#create-commit-signature-protection>
    ///
    /// Create commit signature protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.
    pub async fn post_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks`
    /// * docs <https://docs.github.com/rest/reference/repos#get-status-checks-protection>
    ///
    /// Get status checks protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    pub async fn get_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks`
    /// * docs <https://docs.github.com/rest/reference/repos#update-status-check-protection>
    ///
    /// Update status check protection
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions`
    /// * docs <https://docs.github.com/rest/reference/repos#get-access-restrictions>
    ///
    /// Get access restrictions
    /// Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists who has access to this protected branch.
    ///
    /// **Note**: Users, apps, and teams `restrictions` are only available for organization-owned repositories.
    pub async fn get_repos_owner_repo_branches_branch_protection_restrictions<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRestrictionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictions(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-runs/{check_run_id}`
    /// * docs <https://docs.github.com/rest/reference/checks#get-a-check-run>
    ///
    /// Get a check run
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Gets a single check run using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    pub async fn get_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckRunscheckRunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * patch `/repos/{owner}/{repo}/check-runs/{check_run_id}`
    /// * docs <https://docs.github.com/rest/reference/checks#update-a-check-run>
    ///
    /// Update a check run
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Updates a check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to edit check runs.
    pub async fn patch_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCheckRunscheckRunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * post `/repos/{owner}/{repo}/check-suites`
    /// * docs <https://docs.github.com/rest/reference/checks#create-a-check-suite>
    ///
    /// Create a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/reference/checks#check-runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using "[Update repository preferences for check suites](https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites)". Your GitHub App must have the `checks:write` permission to create check suites.
    pub async fn post_repos_owner_repo_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoCheckSuitesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCheckSuites(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * patch `/repos/{owner}/{repo}/check-suites/preferences`
    /// * docs <https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites>
    ///
    /// Update repository preferences for check suites
    /// Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/reference/checks#create-a-check-suite). You must have admin permissions in the repository to set preferences for check suites.
    pub async fn patch_repos_owner_repo_check_suites_preferences<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCheckSuitesPreferencesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckSuitesPreferences(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-suites/{check_suite_id}`
    /// * docs <https://docs.github.com/rest/reference/checks#get-a-check-suite>
    ///
    /// Get a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// Gets a single check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
    pub async fn get_repos_owner_repo_check_suites_check_suite_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckSuitescheckSuiteIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteId(owner, repo, check_suite_id),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs`
    /// * docs <https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite>
    ///
    /// List check runs in a check suite
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Lists check runs for a check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    pub async fn get_repos_owner_repo_check_suites_check_suite_id_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckSuitescheckSuiteIdCheckRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(
                owner,
                repo,
                check_suite_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}`
    /// * docs <https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-alert>
    ///
    /// Get a code scanning alert
    /// Gets a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint with private repos, the `public_repo` scope also grants permission to read security events on public repos only. GitHub Apps must have the `security_events` read permission to use this endpoint.
    ///
    /// **Deprecation notice**:
    /// The instances field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The same information can now be retrieved via a GET request to the URL specified by `instances_url`.
    pub async fn get_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    /// * tags code-scanning
    /// * patch `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}`
    /// * docs <https://docs.github.com/rest/reference/code-scanning#update-a-code-scanning-alert>
    ///
    /// Update a code scanning alert
    /// Updates the status of a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint with private repositories. You can also use tokens with the `public_repo` scope for public repositories only. GitHub Apps must have the `security_events` write permission to use this endpoint.
    pub async fn patch_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCodeScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}`
    /// * docs <https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository>
    ///
    /// Get a code scanning analysis for a repository
    /// Gets a specified code scanning analysis for a repository.
    /// You must use an access token with the `security_events` scope to use this endpoint with private repos,
    /// the `public_repo` scope also grants permission to read security events on public repos only.
    /// GitHub Apps must have the `security_events` read permission to use this endpoint.
    ///
    /// The default JSON response contains fields that describe the analysis.
    /// This includes the Git reference and commit SHA to which the analysis relates,
    /// the datetime of the analysis, the name of the code scanning tool,
    /// and the number of alerts.
    ///
    /// The `rules_count` field in the default response give the number of rules
    /// that were run in the analysis.
    /// For very old analyses this data is not available,
    /// and `0` is returned in this field.
    ///
    /// If you use the Accept header `application/sarif+json`,
    /// the response contains the analysis data that was uploaded.
    /// This is formatted as
    /// [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html).
    pub async fn get_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningAnalysesanalysisIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
    }
    /// * tags code-scanning
    /// * delete `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}`
    /// * docs <https://docs.github.com/rest/reference/code-scanning#delete-a-code-scanning-analysis-from-a-repository>
    ///
    /// Delete a code scanning analysis from a repository
    /// Deletes a specified code scanning analysis from a repository. For
    /// private repositories, you must use an access token with the `repo` scope. For public repositories,
    /// you must use an access token with `public_repo` scope.
    /// GitHub Apps must have the `security_events` write permission to use this endpoint.
    ///
    /// You can delete one analysis at a time.
    /// To delete a series of analyses, start with the most recent analysis and work backwards.
    /// Conceptually, the process is similar to the undo function in a text editor.
    ///
    /// When you list the analyses for a repository,
    /// one or more will be identified as deletable in the response:
    ///
    /// ```
    /// "deletable": true
    /// ```
    ///
    /// An analysis is deletable when it's the most recent in a set of analyses.
    /// Typically, a repository will have multiple sets of analyses
    /// for each enabled code scanning tool,
    /// where a set is determined by a unique combination of analysis values:
    ///
    /// * `ref`
    /// * `tool`
    /// * `analysis_key`
    /// * `environment`
    ///
    /// If you attempt to delete an analysis that is not the most recent in a set,
    /// you'll get a 400 response with the message:
    ///
    /// ```
    /// Analysis specified is not deletable.
    /// ```
    ///
    /// The response from a successful `DELETE` operation provides you with
    /// two alternative URLs for deleting the next analysis in the set:
    /// `next_analysis_url` and `confirm_delete_url`.
    /// Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
    /// in a set. This is a useful option if you want to preserve at least one analysis
    /// for the specified tool in your repository.
    /// Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
    /// When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url`
    /// in the 200 response is `null`.
    ///
    /// As an example of the deletion process,
    /// let's imagine that you added a workflow that configured a particular code scanning tool
    /// to analyze the code in a repository. This tool has added 15 analyses:
    /// 10 on the default branch, and another 5 on a topic branch.
    /// You therefore have two separate sets of analyses for this tool.
    /// You've now decided that you want to remove all of the analyses for the tool.
    /// To do this you must make 15 separate deletion requests.
    /// To start, you must find an analysis that's identified as deletable.
    /// Each set of analyses always has one that's identified as deletable.
    /// Having found the deletable analysis for one of the two sets,
    /// delete this analysis and then continue deleting the next analysis in the set until they're all deleted.
    /// Then repeat the process for the second set.
    /// The procedure therefore consists of a nested loop:
    ///
    /// **Outer loop**:
    /// * List the analyses for the repository, filtered by tool.
    /// * Parse this list to find a deletable analysis. If found:
    ///
    ///   **Inner loop**:
    ///   * Delete the identified analysis.
    ///   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.
    ///
    /// The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.
    pub async fn delete_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoCodeScanningAnalysesanalysisIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
    }
    /// * tags code-scanning
    /// * get `/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}`
    /// * docs <https://docs.github.com/rest/reference/code-scanning#list-recent-code-scanning-analyses-for-a-repository>
    ///
    /// Get information about a SARIF upload
    /// Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository)." You must use an access token with the `security_events` scope to use this endpoint with private repos, the `public_repo` scope also grants permission to read security events on public repos only. GitHub Apps must have the `security_events` read permission to use this endpoint.
    pub async fn get_repos_owner_repo_code_scanning_sarifs_sarif_id<T, V>(
        &self,
        owner: String,
        repo: String,
        sarif_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningSarifssarifIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningSarifssarifId(owner, repo, sarif_id),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * get `/repos/{owner}/{repo}/codespaces`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-codespaces-in-a-repository-for-the-authenticated-user>
    ///
    /// List codespaces in a repository for the authenticated user
    /// Lists the codespaces associated to a specified repository and the authenticated user.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn get_repos_owner_repo_codespaces<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodespacesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespaces(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * get `/repos/{owner}/{repo}/codespaces/machines`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-available-machine-types-for-a-repository>
    ///
    /// List available machine types for a repository
    /// List the machine types available for a given repository based on its configuration.
    ///
    /// Location is required.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn get_repos_owner_repo_codespaces_machines<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodespacesMachinesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespacesMachines(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/collaborators/{username}/permission`
    /// * docs <https://docs.github.com/rest/reference/repos#get-repository-permissions-for-a-user>
    ///
    /// Get repository permissions for a user
    /// Checks the repository permission of a collaborator. The possible repository permissions are `admin`, `write`, `read`, and `none`.
    pub async fn get_repos_owner_repo_collaborators_username_permission<T, V>(
        &self,
        owner: String,
        repo: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCollaboratorsusernamePermissionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCollaboratorsusernamePermission(owner, repo, username),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-commit-comment>
    ///
    /// Get a commit comment
    ///
    pub async fn get_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-commit-comment>
    ///
    /// Update a commit comment
    ///
    pub async fn patch_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/comments/{comment_id}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-commit-comment>
    ///
    /// Create reaction for a commit comment
    /// Create a reaction to a [commit comment](https://docs.github.com/rest/reference/repos#comments). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
    pub async fn post_repos_owner_repo_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{ref}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-commit>
    ///
    /// Get a commit
    /// Returns the contents of a single commit reference. You must have `read` access for the repository to use this endpoint.
    ///
    /// **Note:** If there are more than 300 files in the commit diff, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.
    ///
    /// You can pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to  fetch `diff` and `patch` formats. Diffs with binary data will have no `patch` property.
    ///
    /// To return only the SHA-1 hash of the commit reference, you can provide the `sha` custom [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) in the `Accept` header. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    pub async fn get_repos_owner_repo_commits_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/commits/{ref}/check-runs`
    /// * docs <https://docs.github.com/rest/reference/checks#list-check-runs-for-a-git-reference>
    ///
    /// List check runs for a Git reference
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
    ///
    /// Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
    pub async fn get_repos_owner_repo_commits_aref_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefCheckRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckRuns(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags checks
    /// * get `/repos/{owner}/{repo}/commits/{ref}/check-suites`
    /// * docs <https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference>
    ///
    /// List check suites for a Git reference
    /// **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
    ///
    /// Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to list check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
    pub async fn get_repos_owner_repo_commits_aref_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefCheckSuitesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckSuites(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/commits/{ref}/status`
    /// * docs <https://docs.github.com/rest/reference/repos#get-the-combined-status-for-a-specific-reference>
    ///
    /// Get the combined status for a specific reference
    /// Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.
    ///
    ///
    /// Additionally, a combined `state` is returned. The `state` is one of:
    ///
    /// *   **failure** if any of the contexts report as `error` or `failure`
    /// *   **pending** if there are no statuses or a context is `pending`
    /// *   **success** if the latest status for all contexts is `success`
    pub async fn get_repos_owner_repo_commits_aref_status<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefStatusResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefStatus(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/community/profile`
    /// * docs <https://docs.github.com/rest/reference/repos#get-community-profile-metrics>
    ///
    /// Get community profile metrics
    /// This endpoint will return all community profile metrics, including an
    /// overall health score, repository description, the presence of documentation, detected
    /// code of conduct, detected license, and the presence of ISSUE\_TEMPLATE, PULL\_REQUEST\_TEMPLATE,
    /// README, and CONTRIBUTING files.
    ///
    /// The `health_percentage` score is defined as a percentage of how many of
    /// these four documents are present: README, CONTRIBUTING, LICENSE, and
    /// CODE_OF_CONDUCT. For example, if all four documents are present, then
    /// the `health_percentage` is `100`. If only one is present, then the
    /// `health_percentage` is `25`.
    ///
    /// `content_reports_enabled` is only returned for organization-owned repositories.
    pub async fn get_repos_owner_repo_community_profile<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommunityProfileResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommunityProfile(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/compare/{basehead}`
    /// * docs <https://docs.github.com/rest/reference/repos#compare-two-commits>
    ///
    /// Compare two commits
    /// The `basehead` param is comprised of two parts: `base` and `head`. Both must be branch names in `repo`. To compare branches across other repositories in the same network as `repo`, use the format `<USERNAME>:branch`.
    ///
    /// The response from the API is equivalent to running the `git log base..head` command; however, commits are returned in chronological order. Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
    ///
    /// The response also includes details on the files that were changed between the two commits. This includes the status of the change (for example, if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.
    ///
    /// **Working with large comparisons**
    ///
    /// To process a response with a large number of commits, you can use (`per_page` or `page`) to paginate the results. When using paging, the list of changed files is only returned with page 1, but includes all changed files for the entire comparison. For more information on working with pagination, see "[Traversing with pagination](/rest/guides/traversing-with-pagination)."
    ///
    /// When calling this API without any paging parameters (`per_page` or `page`), the returned list is limited to 250 commits and the last commit in the list is the most recent of the entire comparison. When a paging parameter is specified, the first commit in the returned list of each page is the earliest.
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    pub async fn get_repos_owner_repo_compare_basehead<T, V>(
        &self,
        owner: String,
        repo: String,
        basehead: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoComparebaseheadResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoComparebasehead(owner, repo, basehead),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/contents/{path}`
    /// * docs <https://docs.github.com/rest/reference/repos#create-or-update-file-contents>
    ///
    /// Create or update file contents
    /// Creates a new file or replaces an existing file in a repository.
    pub async fn put_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoContentspathResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * delete `/repos/{owner}/{repo}/contents/{path}`
    /// * docs <https://docs.github.com/rest/reference/repos#delete-a-file>
    ///
    /// Delete a file
    /// Deletes a file in a repository.
    ///
    /// You can provide an additional `committer` parameter, which is an object containing information about the committer. Or, you can provide an `author` parameter, which is an object containing information about the author.
    ///
    /// The `author` section is optional and is filled in with the `committer` information if omitted. If the `committer` information is omitted, the authenticated user's information is used.
    ///
    /// You must provide values for both `name` and `email`, whether you choose to use `author` or `committer`. Otherwise, you'll receive a `422` status code.
    pub async fn delete_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoContentspathResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments/{deployment_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-deployment>
    ///
    /// Get a deployment
    ///
    pub async fn get_repos_owner_repo_deployments_deployment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoDeploymentsdeploymentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentId(owner, repo, deployment_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-deployment-status>
    ///
    /// Get a deployment status
    /// Users with pull access can view a deployment status for a deployment:
    pub async fn get_repos_owner_repo_deployments_deployment_id_statuses_status_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        status_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoDeploymentsdeploymentIdStatusesstatusIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(
                owner,
                repo,
                deployment_id,
                status_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/environments`
    /// * docs <https://docs.github.com/rest/reference/repos#get-all-environments>
    ///
    /// Get all environments
    /// Get all environments for a repository.
    ///
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_environments<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoEnvironmentsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironments(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/environments/{environment_name}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-an-environment>
    ///
    /// Get an environment
    /// Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
    pub async fn get_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoEnvironmentsenvironmentNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/environments/{environment_name}`
    /// * docs <https://docs.github.com/rest/reference/repos#create-or-update-an-environment>
    ///
    /// Create or update an environment
    /// Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see "[Environments](/actions/reference/environments#environment-protection-rules)."
    ///
    /// **Note:** Although you can use this operation to specify that only branches that match specified name patterns can deploy to this environment, you must use the UI to set the name patterns. For more information, see "[Environments](/actions/reference/environments#deployment-branches)."
    ///
    /// **Note:** To create or update secrets for an environment, see "[Secrets](/rest/reference/actions#secrets)."
    ///
    /// You must authenticate using an access token with the repo scope to use this endpoint.
    pub async fn put_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoEnvironmentsenvironmentNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/blobs/{file_sha}`
    /// * docs <https://docs.github.com/rest/reference/git#get-a-blob>
    ///
    /// Get a blob
    /// The `content` in the response will always be Base64 encoded.
    ///
    /// _Note_: This API supports blobs up to 100 megabytes in size.
    pub async fn get_repos_owner_repo_git_blobs_file_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        file_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitBlobsfileShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitBlobsfileSha(owner, repo, file_sha),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/commits/{commit_sha}`
    /// * docs <https://docs.github.com/rest/reference/git#get-a-commit>
    ///
    /// Get a commit
    /// Gets a Git [commit object](https://git-scm.com/book/en/v1/Git-Internals-Git-Objects#Commit-Objects).
    ///
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    pub async fn get_repos_owner_repo_git_commits_commit_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        commit_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitCommitscommitShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitCommitscommitSha(owner, repo, commit_sha),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/ref/{ref}`
    /// * docs <https://docs.github.com/rest/reference/git#get-a-reference>
    ///
    /// Get a reference
    /// Returns a single reference from your Git database. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't match an existing ref, a `404` is returned.
    ///
    /// **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
    pub async fn get_repos_owner_repo_git_ref_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitRefrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitRefref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * patch `/repos/{owner}/{repo}/git/refs/{ref}`
    /// * docs <https://docs.github.com/rest/reference/git#update-a-reference>
    ///
    /// Update a reference
    ///
    pub async fn patch_repos_owner_repo_git_refs_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoGitRefsrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoGitRefsref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/tags/{tag_sha}`
    /// * docs <https://docs.github.com/rest/reference/git#get-a-tag>
    ///
    /// Get a tag
    /// **Signature verification object**
    ///
    /// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
    ///
    /// | Name | Type | Description |
    /// | ---- | ---- | ----------- |
    /// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
    /// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
    /// | `signature` | `string` | The signature that was extracted from the commit. |
    /// | `payload` | `string` | The value that was signed. |
    ///
    /// These are the possible values for `reason` in the `verification` object:
    ///
    /// | Value | Description |
    /// | ----- | ----------- |
    /// | `expired_key` | The key that made the signature is expired. |
    /// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
    /// | `gpgverify_error` | There was an error communicating with the signature verification service. |
    /// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
    /// | `unsigned` | The object does not include a signature. |
    /// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
    /// | `no_user` | No user was associated with the `committer` email address in the commit. |
    /// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
    /// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
    /// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
    /// | `malformed_signature` | There was an error parsing the signature. |
    /// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
    /// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
    pub async fn get_repos_owner_repo_git_tags_tag_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tag_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitTagstagShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTagstagSha(owner, repo, tag_sha),
            query,
            body,
        )
        .await
    }
    /// * tags git
    /// * get `/repos/{owner}/{repo}/git/trees/{tree_sha}`
    /// * docs <https://docs.github.com/rest/reference/git#get-a-tree>
    ///
    /// Get a tree
    /// Returns a single tree using the SHA1 value for that tree.
    ///
    /// If `truncated` is `true` in the response then the number of items in the `tree` array exceeded our maximum limit. If you need to fetch more items, use the non-recursive method of fetching trees, and fetch one sub-tree at a time.
    pub async fn get_repos_owner_repo_git_trees_tree_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tree_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitTreestreeShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTreestreeSha(owner, repo, tree_sha),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-repository-webhook>
    ///
    /// Get a repository webhook
    /// Returns a webhook configured in a repository. To get only the webhook `config` properties, see "[Get a webhook configuration for a repository](/rest/reference/repos#get-a-webhook-configuration-for-a-repository)."
    pub async fn get_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/hooks/{hook_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-repository-webhook>
    ///
    /// Update a repository webhook
    /// Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for a repository](/rest/reference/repos#update-a-webhook-configuration-for-a-repository)."
    pub async fn patch_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}/config`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-webhook-configuration-for-a-repository>
    ///
    /// Get a webhook configuration for a repository
    /// Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use "[Get a repository webhook](/rest/reference/orgs#get-a-repository-webhook)."
    ///
    /// Access tokens must have the `read:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:read` permission.
    pub async fn get_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/hooks/{hook_id}/config`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-webhook-configuration-for-a-repository>
    ///
    /// Update a webhook configuration for a repository
    /// Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use "[Update a repository webhook](/rest/reference/orgs#update-a-repository-webhook)."
    ///
    /// Access tokens must have the `write:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:write` permission.
    pub async fn patch_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-delivery-for-a-repository-webhook>
    ///
    /// Get a delivery for a repository webhook
    /// Returns a delivery for a webhook configured in a repository.
    pub async fn get_repos_owner_repo_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdDeliveriesdeliveryId(
                owner,
                repo,
                hook_id,
                delivery_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags migrations
    /// * get `/repos/{owner}/{repo}/import`
    /// * docs <https://docs.github.com/rest/reference/migrations#get-an-import-status>
    ///
    /// Get an import status
    /// View the progress of an import.
    ///
    /// **Import status**
    ///
    /// This section includes details about the possible values of the `status` field of the Import Progress response.
    ///
    /// An import that does not have errors will progress through these steps:
    ///
    /// *   `detecting` - the "detection" step of the import is in progress because the request did not include a `vcs` parameter. The import is identifying the type of source control present at the URL.
    /// *   `importing` - the "raw" step of the import is in progress. This is where commit data is fetched from the original repository. The import progress response will include `commit_count` (the total number of raw commits that will be imported) and `percent` (0 - 100, the current progress through the import).
    /// *   `mapping` - the "rewrite" step of the import is in progress. This is where SVN branches are converted to Git branches, and where author updates are applied. The import progress response does not include progress information.
    /// *   `pushing` - the "push" step of the import is in progress. This is where the importer updates the repository on GitHub. The import progress response will include `push_percent`, which is the percent value reported by `git push` when it is "Writing objects".
    /// *   `complete` - the import is complete, and the repository is ready on GitHub.
    ///
    /// If there are problems, you will see one of these in the `status` field:
    ///
    /// *   `auth_failed` - the import requires authentication in order to connect to the original repository. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    /// *   `error` - the import encountered an error. The import progress response will include the `failed_step` and an error message. Contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api) for more information.
    /// *   `detection_needs_auth` - the importer requires authentication for the originating repository to continue detection. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    /// *   `detection_found_nothing` - the importer didn't recognize any source control at the URL. To resolve, [Cancel the import](https://docs.github.com/rest/reference/migrations#cancel-an-import) and [retry](https://docs.github.com/rest/reference/migrations#start-an-import) with the correct URL.
    /// *   `detection_found_multiple` - the importer found several projects or repositories at the provided URL. When this is the case, the Import Progress response will also include a `project_choices` field with the possible project choices as values. To update project choice, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
    ///
    /// **The project_choices field**
    ///
    /// When multiple projects are found at the provided URL, the response hash will include a `project_choices` field, the value of which is an array of hashes each representing a project choice. The exact key/value pairs of the project hashes will differ depending on the version control type.
    ///
    /// **Git LFS related fields**
    ///
    /// This section includes details about Git LFS related fields that may be present in the Import Progress response.
    ///
    /// *   `use_lfs` - describes whether the import has been opted in or out of using Git LFS. The value can be `opt_in`, `opt_out`, or `undecided` if no action has been taken.
    /// *   `has_large_files` - the boolean value describing whether files larger than 100MB were found during the `importing` step.
    /// *   `large_files_size` - the total size in gigabytes of files larger than 100MB found in the originating repository.
    /// *   `large_files_count` - the total number of files larger than 100MB found in the originating repository. To see a list of these files, make a "Get Large Files" request.
    pub async fn get_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoImportResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoImport(owner, repo), query, body)
            .await
    }
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import`
    /// * docs <https://docs.github.com/rest/reference/migrations#update-an-import>
    ///
    /// Update an import
    /// An import can be updated with credentials or a project choice by passing in the appropriate parameters in this API
    /// request. If no parameters are provided, the import will be restarted.
    pub async fn patch_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImport(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import/authors/{author_id}`
    /// * docs <https://docs.github.com/rest/reference/migrations#map-a-commit-author>
    ///
    /// Map a commit author
    /// Update an author's identity for the import. Your application can continue updating authors any time before you push new commits to the repository.
    pub async fn patch_repos_owner_repo_import_authors_author_id<T, V>(
        &self,
        owner: String,
        repo: String,
        author_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportAuthorsauthorIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportAuthorsauthorId(owner, repo, author_id),
            query,
            body,
        )
        .await
    }
    /// * tags migrations
    /// * patch `/repos/{owner}/{repo}/import/lfs`
    /// * docs <https://docs.github.com/rest/reference/migrations#update-git-lfs-preference>
    ///
    /// Update Git LFS preference
    /// You can import repositories from Subversion, Mercurial, and TFS that include files larger than 100MB. This ability is powered by [Git LFS](https://git-lfs.github.com). You can learn more about our LFS feature and working with large files [on our help site](https://help.github.com/articles/versioning-large-files/).
    pub async fn patch_repos_owner_repo_import_lfs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportLfsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportLfs(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags apps
    /// * get `/repos/{owner}/{repo}/installation`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-repository-installation-for-the-authenticated-app>
    ///
    /// Get a repository installation for the authenticated app
    /// Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_repos_owner_repo_installation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoInstallation(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags interactions
    /// * put `/repos/{owner}/{repo}/interaction-limits`
    /// * docs <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-a-repository>
    ///
    /// Set interaction restrictions for a repository
    /// Temporarily restricts interactions to a certain type of GitHub user within the given repository. You must have owner or admin access to set these restrictions. If an interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
    pub async fn put_repos_owner_repo_interaction_limits<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoInteractionLimits(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/invitations/{invitation_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-repository-invitation>
    ///
    /// Update a repository invitation
    ///
    pub async fn patch_repos_owner_repo_invitations_invitation_id<T, V>(
        &self,
        owner: String,
        repo: String,
        invitation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoInvitationsinvitationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoInvitationsinvitationId(owner, repo, invitation_id),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/issues#get-an-issue-comment>
    ///
    /// Get an issue comment
    ///
    pub async fn get_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/issues/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/issues#update-an-issue-comment>
    ///
    /// Update an issue comment
    ///
    pub async fn patch_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoIssuesCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue-comment>
    ///
    /// Create reaction for an issue comment
    /// Create a reaction to an [issue comment](https://docs.github.com/rest/reference/issues#comments). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.
    pub async fn post_repos_owner_repo_issues_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoIssuesCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/events/{event_id}`
    /// * docs <https://docs.github.com/rest/reference/issues#get-an-issue-event>
    ///
    /// Get an issue event
    ///
    pub async fn get_repos_owner_repo_issues_events_event_id<T, V>(
        &self,
        owner: String,
        repo: String,
        event_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesEventseventIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesEventseventId(owner, repo, event_id),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/issues/{issue_number}`
    /// * docs <https://docs.github.com/rest/reference/issues#get-an-issue>
    ///
    /// Get an issue
    /// The API returns a [`301 Moved Permanently` status](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-redirects-redirects) if the issue was
    /// [transferred](https://help.github.com/articles/transferring-an-issue-to-another-repository/) to another repository. If
    /// the issue was transferred to or deleted from a repository where the authenticated user lacks read access, the API
    /// returns a `404 Not Found` status. If the issue was deleted from a repository where the authenticated user has read
    /// access, the API returns a `410 Gone` status. To receive webhook events for transferred and deleted issues, subscribe
    /// to the [`issues`](https://docs.github.com/webhooks/event-payloads/#issues) webhook.
    ///
    /// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
    /// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
    /// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
    /// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
    pub async fn get_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesissueNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/issues/{issue_number}`
    /// * docs <https://docs.github.com/rest/reference/issues/#update-an-issue>
    ///
    /// Update an issue
    /// Issue owners and users with push access can edit an issue.
    pub async fn patch_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoIssuesissueNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * delete `/repos/{owner}/{repo}/issues/{issue_number}/assignees`
    /// * docs <https://docs.github.com/rest/reference/issues#remove-assignees-from-an-issue>
    ///
    /// Remove assignees from an issue
    /// Removes one or more assignees from an issue.
    pub async fn delete_repos_owner_repo_issues_issue_number_assignees<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoIssuesissueNumberAssigneesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoIssuesissueNumberAssignees(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/issues/{issue_number}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-an-issue>
    ///
    /// Create reaction for an issue
    /// Create a reaction to an [issue](https://docs.github.com/rest/reference/issues/). A response with an HTTP `200` status means that you already added the reaction type to this issue.
    pub async fn post_repos_owner_repo_issues_issue_number_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoIssuesissueNumberReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesissueNumberReactions(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/keys/{key_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-deploy-key>
    ///
    /// Get a deploy key
    ///
    pub async fn get_repos_owner_repo_keys_key_id<T, V>(
        &self,
        owner: String,
        repo: String,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoKeyskeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoKeyskeyId(owner, repo, key_id),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/labels/{name}`
    /// * docs <https://docs.github.com/rest/reference/issues#get-a-label>
    ///
    /// Get a label
    ///
    pub async fn get_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/labels/{name}`
    /// * docs <https://docs.github.com/rest/reference/issues#update-a-label>
    ///
    /// Update a label
    ///
    pub async fn patch_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
    }
    /// * tags licenses
    /// * get `/repos/{owner}/{repo}/license`
    /// * docs <https://docs.github.com/rest/reference/licenses/#get-the-license-for-a-repository>
    ///
    /// Get the license for a repository
    /// This method returns the contents of the repository's license file, if one is detected.
    ///
    /// Similar to [Get repository content](https://docs.github.com/rest/reference/repos#get-repository-content), this method also supports [custom media types](https://docs.github.com/rest/overview/media-types) for retrieving the raw license content or rendered license HTML.
    pub async fn get_repos_owner_repo_license<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoLicenseResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLicense(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/merge-upstream`
    /// * docs <https://docs.github.com/rest/reference/repos#sync-a-fork-branch-with-the-upstream-repository>
    ///
    /// Sync a fork branch with the upstream repository
    /// Sync a branch of a forked repository to keep it up-to-date with the upstream repository.
    pub async fn post_repos_owner_repo_merge_upstream<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoMergeUpstreamResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoMergeUpstream(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * get `/repos/{owner}/{repo}/milestones/{milestone_number}`
    /// * docs <https://docs.github.com/rest/reference/issues#get-a-milestone>
    ///
    /// Get a milestone
    ///
    pub async fn get_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoMilestonesmilestoneNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
    }
    /// * tags issues
    /// * patch `/repos/{owner}/{repo}/milestones/{milestone_number}`
    /// * docs <https://docs.github.com/rest/reference/issues#update-a-milestone>
    ///
    /// Update a milestone
    ///
    pub async fn patch_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoMilestonesmilestoneNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-github-pages-site>
    ///
    /// Get a GitHub Pages site
    ///
    pub async fn get_repos_owner_repo_pages<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoPages(owner, repo), query, body)
            .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/builds/latest`
    /// * docs <https://docs.github.com/rest/reference/repos#get-latest-pages-build>
    ///
    /// Get latest Pages build
    ///
    pub async fn get_repos_owner_repo_pages_builds_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesBuildsLatestResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsLatest(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/builds/{build_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-github-pages-build>
    ///
    /// Get GitHub Pages build
    ///
    pub async fn get_repos_owner_repo_pages_builds_build_id<T, V>(
        &self,
        owner: String,
        repo: String,
        build_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesBuildsbuildIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsbuildId(owner, repo, build_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/pages/health`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-dns-health-check-for-github-pages>
    ///
    /// Get a DNS health check for GitHub Pages
    /// Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.
    ///
    /// The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.
    ///
    /// Users must have admin or owner permissions. GitHub Apps must have the `pages:write` and `administration:write` permission to use this endpoint.
    pub async fn get_repos_owner_repo_pages_health<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesHealthResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesHealth(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/pulls#get-a-review-comment-for-a-pull-request>
    ///
    /// Get a review comment for a pull request
    /// Provides details for a review comment.
    pub async fn get_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullsCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * patch `/repos/{owner}/{repo}/pulls/comments/{comment_id}`
    /// * docs <https://docs.github.com/rest/reference/pulls#update-a-review-comment-for-a-pull-request>
    ///
    /// Update a review comment for a pull request
    /// Enables you to edit a review comment.
    pub async fn patch_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoPullsCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions#create-reaction-for-a-pull-request-review-comment>
    ///
    /// Create reaction for a pull request review comment
    /// Create a reaction to a [pull request review comment](https://docs.github.com/rest/reference/pulls#comments). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.
    pub async fn post_repos_owner_repo_pulls_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullsCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullsCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}`
    /// * docs <https://docs.github.com/rest/reference/pulls#get-a-pull-request>
    ///
    /// Get a pull request
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Lists details of a pull request by providing its number.
    ///
    /// When you get, [create](https://docs.github.com/rest/reference/pulls/#create-a-pull-request), or [edit](https://docs.github.com/rest/reference/pulls#update-a-pull-request) a pull request, GitHub creates a merge commit to test whether the pull request can be automatically merged into the base branch. This test commit is not added to the base branch or the head branch. You can review the status of the test commit using the `mergeable` key. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
    ///
    /// The value of the `mergeable` attribute can be `true`, `false`, or `null`. If the value is `null`, then GitHub has started a background job to compute the mergeability. After giving the job time to complete, resubmit the request. When the job finishes, you will see a non-`null` value for the `mergeable` attribute in the response. If `mergeable` is `true`, then `merge_commit_sha` will be the SHA of the _test_ merge commit.
    ///
    /// The value of the `merge_commit_sha` attribute changes depending on the state of the pull request. Before merging a pull request, the `merge_commit_sha` attribute holds the SHA of the _test_ merge commit. After merging a pull request, the `merge_commit_sha` attribute changes depending on how you merged the pull request:
    ///
    /// *   If merged as a [merge commit](https://help.github.com/articles/about-merge-methods-on-github/), `merge_commit_sha` represents the SHA of the merge commit.
    /// *   If merged via a [squash](https://help.github.com/articles/about-merge-methods-on-github/#squashing-your-merge-commits), `merge_commit_sha` represents the SHA of the squashed commit on the base branch.
    /// *   If [rebased](https://help.github.com/articles/about-merge-methods-on-github/#rebasing-and-merging-your-commits), `merge_commit_sha` represents the commit that the base branch was updated to.
    ///
    /// Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
    pub async fn get_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * patch `/repos/{owner}/{repo}/pulls/{pull_number}`
    /// * docs <https://docs.github.com/rest/reference/pulls/#update-a-pull-request>
    ///
    /// Update a pull request
    /// Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
    pub async fn patch_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoPullspullNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/merge`
    /// * docs <https://docs.github.com/rest/reference/pulls#merge-a-pull-request>
    ///
    /// Merge a pull request
    /// This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    pub async fn put_repos_owner_repo_pulls_pull_number_merge<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberMergeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberMerge(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`
    /// * docs <https://docs.github.com/rest/reference/pulls#list-requested-reviewers-for-a-pull-request>
    ///
    /// List requested reviewers for a pull request
    ///
    pub async fn get_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberRequestedReviewersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberRequestedReviewers(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * delete `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`
    /// * docs <https://docs.github.com/rest/reference/pulls#remove-requested-reviewers-from-a-pull-request>
    ///
    /// Remove requested reviewers from a pull request
    ///
    pub async fn delete_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoPullspullNumberRequestedReviewersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberRequestedReviewers(
                owner,
                repo,
                pull_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/reviews`
    /// * docs <https://docs.github.com/rest/reference/pulls#create-a-review-for-a-pull-request>
    ///
    /// Create a review for a pull request
    /// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
    ///
    /// Pull request reviews created in the `PENDING` state do not include the `submitted_at` property in the response.
    ///
    /// **Note:** To comment on a specific line in a file, you need to first determine the _position_ of that line in the diff. The GitHub REST API v3 offers the `application/vnd.github.v3.diff` [media type](https://docs.github.com/rest/overview/media-types#commits-commit-comparison-and-pull-requests). To see a pull request diff, add this media type to the `Accept` header of a call to the [single pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) endpoint.
    ///
    /// The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullspullNumberReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviews(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * get `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs <https://docs.github.com/rest/reference/pulls#get-a-review-for-a-pull-request>
    ///
    /// Get a review for a pull request
    ///
    pub async fn get_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs <https://docs.github.com/rest/reference/pulls#update-a-review-for-a-pull-request>
    ///
    /// Update a review for a pull request
    /// Update the review summary comment with new text.
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * delete `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`
    /// * docs <https://docs.github.com/rest/reference/pulls#delete-a-pending-review-for-a-pull-request>
    ///
    /// Delete a pending review for a pull request
    ///
    pub async fn delete_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * put `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals`
    /// * docs <https://docs.github.com/rest/reference/pulls#dismiss-a-review-for-a-pull-request>
    ///
    /// Dismiss a review for a pull request
    /// **Note:** To dismiss a pull request review on a [protected branch](https://docs.github.com/rest/reference/repos#branches), you must be a repository administrator or be included in the list of people or teams who can dismiss pull request reviews.
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id_dismissals<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberReviewsreviewIdDismissalsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags pulls
    /// * post `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events`
    /// * docs <https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request>
    ///
    /// Submit a review for a pull request
    ///
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews_review_id_events<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullspullNumberReviewsreviewIdEventsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviewsreviewIdEvents(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/readme`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-repository-readme>
    ///
    /// Get a repository README
    /// Gets the preferred README for a repository.
    ///
    /// READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
    pub async fn get_repos_owner_repo_readme<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReadmeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoReadme(owner, repo), query, body)
            .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/readme/{dir}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-repository-directory-readme>
    ///
    /// Get a repository README for a directory
    /// Gets the README from a repository directory.
    ///
    /// READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
    pub async fn get_repos_owner_repo_readme_dir<T, V>(
        &self,
        owner: String,
        repo: String,
        dir: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReadmedirResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReadmedir(owner, repo, dir),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/assets/{asset_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-release-asset>
    ///
    /// Get a release asset
    /// To download the asset's binary content, set the `Accept` header of the request to [`application/octet-stream`](https://docs.github.com/rest/overview/media-types). The API will either redirect the client to the location, or stream it directly if possible. API clients should handle both a `200` or `302` response.
    pub async fn get_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesAssetsassetIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/releases/assets/{asset_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-release-asset>
    ///
    /// Update a release asset
    /// Users with push access to the repository can edit a release asset.
    pub async fn patch_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoReleasesAssetsassetIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * post `/repos/{owner}/{repo}/releases/generate-notes`
    /// * docs <https://docs.github.com/rest/reference/repos#generate-release-notes>
    ///
    /// Generate release notes content for a release
    /// Generate a name and body describing a [release](https://docs.github.com/rest/reference/repos#releases). The body content will be markdown formatted and contain information like the changes since last release and users who contributed. The generated release notes are not saved anywhere. They are intended to be generated and used when creating a new release.
    pub async fn post_repos_owner_repo_releases_generate_notes<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoReleasesGenerateNotesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesGenerateNotes(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/latest`
    /// * docs <https://docs.github.com/rest/reference/repos#get-the-latest-release>
    ///
    /// Get the latest release
    /// View the latest published full release for the repository.
    ///
    /// The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.
    pub async fn get_repos_owner_repo_releases_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesLatestResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesLatest(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/tags/{tag}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-release-by-tag-name>
    ///
    /// Get a release by tag name
    /// Get a published release with the specified tag.
    pub async fn get_repos_owner_repo_releases_tags_tag<T, V>(
        &self,
        owner: String,
        repo: String,
        tag: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesTagstagResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesTagstag(owner, repo, tag),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/releases/{release_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#get-a-release>
    ///
    /// Get a release
    /// **Note:** This returns an `upload_url` key corresponding to the endpoint for uploading release assets. This key is a [hypermedia resource](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia).
    pub async fn get_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesreleaseIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * patch `/repos/{owner}/{repo}/releases/{release_id}`
    /// * docs <https://docs.github.com/rest/reference/repos#update-a-release>
    ///
    /// Update a release
    /// Users with push access to the repository can edit a release.
    pub async fn patch_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoReleasesreleaseIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    /// * tags reactions
    /// * post `/repos/{owner}/{repo}/releases/{release_id}/reactions`
    /// * docs <https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-release>
    ///
    /// Create reaction for a release
    /// Create a reaction to a [release](https://docs.github.com/rest/reference/repos#releases). A response with a `Status: 200 OK` means that you already added the reaction type to this release.
    pub async fn post_repos_owner_repo_releases_release_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoReleasesreleaseIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesreleaseIdReactions(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    /// * tags secret-scanning
    /// * get `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}`
    /// * docs <https://docs.github.com/rest/reference/secret-scanning#get-a-secret-scanning-alert>
    ///
    /// Get a secret scanning alert
    /// Gets a single secret scanning alert detected in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` read permission to use this endpoint.
    pub async fn get_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoSecretScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSecretScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    /// * tags secret-scanning
    /// * patch `/repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}`
    /// * docs <https://docs.github.com/rest/reference/secret-scanning#update-a-secret-scanning-alert>
    ///
    /// Update a secret scanning alert
    /// Updates the status of a secret scanning alert in a private repository. To use this endpoint, you must be an administrator for the repository or organization, and you must use an access token with the `repo` scope or `security_events` scope.
    ///
    /// GitHub Apps must have the `secret_scanning_alerts` write permission to use this endpoint.
    pub async fn patch_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoSecretScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoSecretScanningAlertsalertNumber(
                owner,
                repo,
                alert_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/stats/participation`
    /// * docs <https://docs.github.com/rest/reference/repos#get-the-weekly-commit-count>
    ///
    /// Get the weekly commit count
    /// Returns the total commit counts for the `owner` and total commit counts in `all`. `all` is everyone combined, including the `owner` in the last 52 weeks. If you'd like to get the commit counts for non-owners, you can subtract `owner` from `all`.
    ///
    /// The array order is oldest week (index 0) to most recent week.
    pub async fn get_repos_owner_repo_stats_participation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoStatsParticipationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoStatsParticipation(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags activity
    /// * get `/repos/{owner}/{repo}/subscription`
    /// * docs <https://docs.github.com/rest/reference/activity#get-a-repository-subscription>
    ///
    /// Get a repository subscription
    ///
    pub async fn get_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags activity
    /// * put `/repos/{owner}/{repo}/subscription`
    /// * docs <https://docs.github.com/rest/reference/activity#set-a-repository-subscription>
    ///
    /// Set a repository subscription
    /// If you would like to watch a repository, set `subscribed` to `true`. If you would like to ignore notifications made within a repository, set `ignored` to `true`. If you would like to stop watching a repository, [delete the repository's subscription](https://docs.github.com/rest/reference/activity#delete-a-repository-subscription) completely.
    pub async fn put_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/topics`
    /// * docs <https://docs.github.com/rest/reference/repos#get-all-repository-topics>
    ///
    /// Get all repository topics
    ///
    pub async fn get_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoTopics(owner, repo), query, body)
            .await
    }
    /// * tags repos
    /// * put `/repos/{owner}/{repo}/topics`
    /// * docs <https://docs.github.com/rest/reference/repos#replace-all-repository-topics>
    ///
    /// Replace all repository topics
    ///
    pub async fn put_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutReposownerrepoTopics(owner, repo), query, body)
            .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/clones`
    /// * docs <https://docs.github.com/rest/reference/repos#get-repository-clones>
    ///
    /// Get repository clones
    /// Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
    pub async fn get_repos_owner_repo_traffic_clones<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTrafficClonesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficClones(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags repos
    /// * get `/repos/{owner}/{repo}/traffic/views`
    /// * docs <https://docs.github.com/rest/reference/repos#get-page-views>
    ///
    /// Get page views
    /// Get the total number of views and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
    pub async fn get_repos_owner_repo_traffic_views<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTrafficViewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficViews(owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets`
    /// * docs <https://docs.github.com/rest/reference/actions#list-environment-secrets>
    ///
    /// List environment secrets
    /// Lists all secrets available in an environment without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repositories_repository_id_environments_environment_name_secrets<T, V>(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets/public-key`
    /// * docs <https://docs.github.com/rest/reference/actions#get-an-environment-public-key>
    ///
    /// Get an environment public key
    /// Get the public key for an environment, which you need to encrypt environment secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_public_key<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags actions
    /// * get `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}`
    /// * docs <https://docs.github.com/rest/reference/actions#get-an-environment-secret>
    ///
    /// Get an environment secret
    /// Gets a single environment secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_secret_name<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretNameResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(
                repository_id,
                environment_name,
                secret_name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Groups`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-provisioned-scim-groups-for-an-enterprise>
    ///
    /// List provisioned SCIM groups for an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    pub async fn get_scim_v2_enterprises_enterprise_Groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroups(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-group>
    ///
    /// Get SCIM provisioning information for an enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    pub async fn get_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * put `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-group>
    ///
    /// Set SCIM information for a provisioned enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Replaces an existing provisioned group’s information. You must provide all the information required for the group as if you were provisioning it for the first time. Any existing group information that you don't provide will be removed, including group membership. If you want to only update a specific attribute, use the [Update an attribute for a SCIM enterprise group](#update-an-attribute-for-a-scim-enterprise-group) endpoint instead.
    pub async fn put_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * patch `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-group>
    ///
    /// Update an attribute for a SCIM enterprise group
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Allows you to change a provisioned group’s individual attributes. To change a group’s values, you must provide a specific Operations JSON format that contains at least one of the add, remove, or replace operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    pub async fn patch_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Users`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#list-scim-provisioned-identities-for-an-enterprise>
    ///
    /// List SCIM provisioned identities for an enterprise
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Retrieves a paginated list of all provisioned enterprise members, including pending invitations.
    ///
    /// When a user with a SAML-provisioned external identity leaves (or is removed from) an enterprise, the account's metadata is immediately removed. However, the returned list of user accounts might not always match the organization or enterprise member list you see on GitHub. This can happen in certain cases where an external identity associated with an organization will not match an organization member:
    ///   - When a user with a SCIM-provisioned external identity is removed from an enterprise, the account's metadata is preserved to allow the user to re-join the organization in the future.
    ///   - When inviting a user to join an organization, you can expect to see their external identity in the results before they accept the invitation, or if the invitation is cancelled (or never accepted).
    ///   - When a user is invited over SCIM, an external identity is created that matches with the invitee's email address. However, this identity is only linked to a user account when the user accepts the invitation by going through SAML SSO.
    ///
    /// The returned list of external identities can include an entry for a `null` user. These are unlinked SAML identities that are created when a user goes through the following Single Sign-On (SSO) process but does not sign in to their GitHub account after completing SSO:
    ///
    /// 1. The user is granted access by the IdP and is not a member of the GitHub enterprise.
    ///
    /// 1. The user attempts to access the GitHub enterprise and initiates the SAML SSO process, and is not currently signed in to their GitHub account.
    ///
    /// 1. After successfully authenticating with the SAML SSO IdP, the `null` external identity entry is created and the user is prompted to sign in to their GitHub account:
    ///    - If the user signs in, their GitHub account is linked to this entry.
    ///    - If the user does not sign in (or does not create a new account when prompted), they are not added to the GitHub enterprise, and the external identity `null` entry remains in place.
    pub async fn get_scim_v2_enterprises_enterprise_Users<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseUsersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsers(enterprise),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * get `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-user>
    ///
    /// Get SCIM provisioning information for an enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    pub async fn get_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * put `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-user>
    ///
    /// Set SCIM information for a provisioned enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](#update-an-attribute-for-an-enterprise-scim-user) endpoint instead.
    ///
    /// You must at least provide the required values for the user: `userName`, `name`, and `emails`.
    ///
    /// **Warning:** Setting `active: false` removes the user from the enterprise, deletes the external identity, and deletes the associated `{scim_user_id}`.
    pub async fn put_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    /// * tags enterprise-admin
    /// * patch `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}`
    /// * docs <https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-user>
    ///
    /// Update an attribute for a SCIM enterprise user
    /// **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
    ///
    /// Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    ///
    /// **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
    ///
    /// **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the enterprise, deletes the external identity, and deletes the associated `:scim_user_id`.
    ///
    /// ```
    /// {
    ///   "Operations":[{
    ///     "op":"replace",
    ///     "value":{
    ///       "active":false
    ///     }
    ///   }]
    /// }
    /// ```
    pub async fn patch_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    /// * tags search
    /// * get `/search/code`
    /// * docs <https://docs.github.com/rest/reference/search#search-code>
    ///
    /// Search code
    /// Searches for query terms inside of a file. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for code, you can get text match metadata for the file **content** and file **path** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find the definition of the `addClass` function inside [jQuery](https://github.com/jquery/jquery) repository, your query would look something like this:
    ///
    /// `q=addClass+in:file+language:js+repo:jquery/jquery`
    ///
    /// This query searches for the keyword `addClass` within a file's contents. The query limits the search to files where the language is JavaScript in the `jquery/jquery` repository.
    ///
    /// #### Considerations for code search
    ///
    /// Due to the complexity of searching code, there are a few restrictions on how searches are performed:
    ///
    /// *   Only the _default branch_ is considered. In most cases, this will be the `master` branch.
    /// *   Only files smaller than 384 KB are searchable.
    /// *   You must always include at least one search term when searching source code. For example, searching for [`language:go`](https://github.com/search?utf8=%E2%9C%93&q=language%3Ago&type=Code) is not valid, while [`amazing
    /// language:go`](https://github.com/search?utf8=%E2%9C%93&q=amazing+language%3Ago&type=Code) is.
    pub async fn get_search_code<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchCodeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCode(), query, body).await
    }
    /// * tags search
    /// * get `/search/commits`
    /// * docs <https://docs.github.com/rest/reference/search#search-commits>
    ///
    /// Search commits
    /// Find commits via various criteria on the default branch (usually `master`). This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for commits, you can get text match metadata for the **message** field when you provide the `text-match` media type. For more details about how to receive highlighted search results, see [Text match
    /// metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find commits related to CSS in the [octocat/Spoon-Knife](https://github.com/octocat/Spoon-Knife) repository. Your query would look something like this:
    ///
    /// `q=repo:octocat/Spoon-Knife+css`
    pub async fn get_search_commits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchCommitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCommits(), query, body).await
    }
    /// * tags search
    /// * get `/search/issues`
    /// * docs <https://docs.github.com/rest/reference/search#search-issues-and-pull-requests>
    ///
    /// Search issues and pull requests
    /// Find issues by state and keyword. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for issues, you can get text match metadata for the issue **title**, issue **body**, and issue **comment body** fields when you pass the `text-match` media type. For more details about how to receive highlighted
    /// search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find the oldest unresolved Python bugs on Windows. Your query might look something like this.
    ///
    /// `q=windows+label:bug+language:python+state:open&sort=created&order=asc`
    ///
    /// This query searches for the keyword `windows`, within any open issue that is labeled as `bug`. The search runs across repositories whose primary language is Python. The results are sorted by creation date in ascending order, which means the oldest issues appear first in the search results.
    ///
    /// **Note:** For [user-to-server](https://docs.github.com/developers/apps/identifying-and-authorizing-users-for-github-apps#user-to-server-requests) GitHub App requests, you can't retrieve a combination of issues and pull requests in a single query. Requests that don't include the `is:issue` or `is:pull-request` qualifier will receive an HTTP `422 Unprocessable Entity` response. To get results for both issues and pull requests, you must send separate queries for issues and pull requests. For more information about the `is` qualifier, see "[Searching only issues or pull requests](https://docs.github.com/github/searching-for-information-on-github/searching-issues-and-pull-requests#search-only-issues-or-pull-requests)."
    pub async fn get_search_issues<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchIssuesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchIssues(), query, body).await
    }
    /// * tags search
    /// * get `/search/labels`
    /// * docs <https://docs.github.com/rest/reference/search#search-labels>
    ///
    /// Search labels
    /// Find labels in a repository with names or descriptions that match search keywords. Returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for labels, you can get text match metadata for the label **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to find labels in the `linguist` repository that match `bug`, `defect`, or `enhancement`. Your query might look like this:
    ///
    /// `q=bug+defect+enhancement&repository_id=64778136`
    ///
    /// The labels that best match the query appear first in the search results.
    pub async fn get_search_labels<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchLabels(), query, body).await
    }
    /// * tags search
    /// * get `/search/repositories`
    /// * docs <https://docs.github.com/rest/reference/search#search-repositories>
    ///
    /// Search repositories
    /// Find repositories via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for repositories, you can get text match metadata for the **name** and **description** fields when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to search for popular Tetris repositories written in assembly code, your query might look like this:
    ///
    /// `q=tetris+language:assembly&sort=stars&order=desc`
    ///
    /// This query searches for repositories with the word `tetris` in the name, the description, or the README. The results are limited to repositories where the primary language is assembly. The results are sorted by stars in descending order, so that the most popular repositories appear first in the search results.
    pub async fn get_search_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchRepositories(), query, body)
            .await
    }
    /// * tags search
    /// * get `/search/topics`
    /// * docs <https://docs.github.com/rest/reference/search#search-topics>
    ///
    /// Search topics
    /// Find topics via various criteria. Results are sorted by best match. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination). See "[Searching topics](https://help.github.com/articles/searching-topics/)" for a detailed list of qualifiers.
    ///
    /// When searching for topics, you can get text match metadata for the topic's **short\_description**, **description**, **name**, or **display\_name** field when you pass the `text-match` media type. For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you want to search for topics related to Ruby that are featured on https://github.com/topics. Your query might look like this:
    ///
    /// `q=ruby+is:featured`
    ///
    /// This query searches for topics with the keyword `ruby` and limits the results to find only topics that are featured. The topics that are the best match for the query appear first in the search results.
    pub async fn get_search_topics<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchTopics(), query, body).await
    }
    /// * tags search
    /// * get `/search/users`
    /// * docs <https://docs.github.com/rest/reference/search#search-users>
    ///
    /// Search users
    /// Find users via various criteria. This method returns up to 100 results [per page](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination).
    ///
    /// When searching for users, you can get text match metadata for the issue **login**, **email**, and **name** fields when you pass the `text-match` media type. For more details about highlighting search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata). For more details about how to receive highlighted search results, see [Text match metadata](https://docs.github.com/rest/reference/search#text-match-metadata).
    ///
    /// For example, if you're looking for a list of popular users, you might try this query:
    ///
    /// `q=tom+repos:%3E42+followers:%3E1000`
    ///
    /// This query searches for users with the name `tom`. The results are restricted to users with more than 42 repositories and over 1,000 followers.
    pub async fn get_search_users<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchUsersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchUsers(), query, body).await
    }
    /// * tags teams
    /// * get `/teams/{team_id}`
    /// * docs <https://docs.github.com/rest/reference/teams/#get-a-team-legacy>
    ///
    /// Get a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/reference/teams#get-a-team-by-name) endpoint.
    pub async fn get_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetTeamsteamId(team_id), query, body)
            .await
    }
    /// * tags teams
    /// * patch `/teams/{team_id}`
    /// * docs <https://docs.github.com/rest/reference/teams/#update-a-team-legacy>
    ///
    /// Update a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/reference/teams#update-a-team) endpoint.
    ///
    /// To edit a team, the authenticated user must either be an organization owner or a team maintainer.
    ///
    /// **Note:** With nested teams, the `privacy` for parent teams cannot be `secret`.
    pub async fn patch_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchTeamsteamId(team_id), query, body)
            .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/discussions/{discussion_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-a-discussion-legacy>
    ///
    /// Get a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion](https://docs.github.com/rest/reference/teams#get-a-discussion) endpoint.
    ///
    /// Get a specific discussion on a team's page. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn get_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/teams/{team_id}/discussions/{discussion_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#update-a-discussion-legacy>
    ///
    /// Update a discussion (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion](https://docs.github.com/rest/reference/teams#update-a-discussion) endpoint.
    ///
    /// Edits the title and body text of a discussion post. Only the parameters you provide are updated. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn patch_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-a-discussion-comment-legacy>
    ///
    /// Get a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion comment](https://docs.github.com/rest/reference/teams#get-a-discussion-comment) endpoint.
    ///
    /// Get a specific comment on a team discussion. OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn get_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}`
    /// * docs <https://docs.github.com/rest/reference/teams#update-a-discussion-comment-legacy>
    ///
    /// Update a discussion comment (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion comment](https://docs.github.com/rest/reference/teams#update-a-discussion-comment) endpoint.
    ///
    /// Edits the body text of a discussion comment. OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn patch_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user-legacy>
    ///
    /// Get team membership for a user (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/reference/teams#get-team-membership-for-a-user) endpoint.
    ///
    /// Team members will include the members of child teams.
    ///
    /// To get a user's membership with a team, the team must be visible to the authenticated user.
    ///
    /// **Note:**
    /// The response contains the `state` of the membership and the member's `role`.
    ///
    /// The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/reference/teams#create-a-team).
    pub async fn get_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * put `/teams/{team_id}/memberships/{username}`
    /// * docs <https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user-legacy>
    ///
    /// Add or update team membership for a user (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/reference/teams#add-or-update-team-membership-for-a-user) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.
    ///
    /// **Note:** When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see "[Synchronizing teams between your identity provider and GitHub](https://help.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/)."
    ///
    /// If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the "pending" state until the user accepts the invitation, at which point the membership will transition to the "active" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.
    ///
    /// If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.
    pub async fn put_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutTeamsteamIdMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/projects/{project_id}`
    /// * docs <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-project-legacy>
    ///
    /// Check team permissions for a project (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a project](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-project) endpoint.
    ///
    /// Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.
    pub async fn get_teams_team_id_projects_project_id<T, V>(
        &self,
        team_id: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdProjectsprojectId(team_id, project_id),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/repos/{owner}/{repo}`
    /// * docs <https://docs.github.com/rest/reference/teams/#check-team-permissions-for-a-repository-legacy>
    ///
    /// Check team permissions for a repository (Legacy)
    /// **Note**: Repositories inherited through a parent team will also be checked.
    ///
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/reference/teams#check-team-permissions-for-a-repository) endpoint.
    ///
    /// You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/overview/media-types/) via the `Accept` header:
    pub async fn get_teams_team_id_repos_owner_repo<T, V>(
        &self,
        team_id: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdReposownerrepo(team_id, owner, repo),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * get `/teams/{team_id}/team-sync/group-mappings`
    /// * docs <https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team-legacy>
    ///
    /// List IdP groups for a team (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List IdP groups for a team`](https://docs.github.com/rest/reference/teams#list-idp-groups-for-a-team) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// List IdP groups connected to a team on GitHub.
    pub async fn get_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
    }
    /// * tags teams
    /// * patch `/teams/{team_id}/team-sync/group-mappings`
    /// * docs <https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections-legacy>
    ///
    /// Create or update IdP group connections (Legacy)
    /// **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create or update IdP group connections`](https://docs.github.com/rest/reference/teams#create-or-update-idp-group-connections) endpoint.
    ///
    /// Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
    ///
    /// Creates, updates, or removes a connection between a team and an IdP group. When adding groups to a team, you must include all new and existing groups to avoid replacing existing groups with the new ones. Specifying an empty `groups` array will remove all connections for a team.
    pub async fn patch_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
    }
    /// * tags users
    /// * patch `/user`
    /// * docs <https://docs.github.com/rest/reference/users/#update-the-authenticated-user>
    ///
    /// Update the authenticated user
    /// **Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.
    pub async fn patch_user<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUser(), query, body).await
    }
    /// * tags codespaces
    /// * get `/user/codespaces`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-codespaces-for-the-authenticated-user>
    ///
    /// List codespaces for the authenticated user
    /// Lists the authenticated user's codespaces.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn get_user_codespaces<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespaces(), query, body).await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/secrets`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-secrets-for-the-authenticated-user>
    ///
    /// List secrets for the authenticated user
    /// Lists all secrets available for a user's Codespaces without revealing their
    /// encrypted values.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    pub async fn get_user_codespaces_secrets<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecrets(), query, body)
            .await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/public-key`
    /// * docs <https://docs.github.com/rest/reference/codespaces#get-public-key-for-the-authenticated-user>
    ///
    /// Get public key for the authenticated user
    /// Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with one of the 'read:user' or 'user' scopes in their personal access token. User must have Codespaces access to use this endpoint.
    pub async fn get_user_codespaces_secrets_public_key<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecretsPublicKey(), query, body)
            .await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/{secret_name}`
    /// * docs <https://docs.github.com/rest/reference/codespaces#get-a-secret-for-the-authenticated-user>
    ///
    /// Get a secret for the authenticated user
    /// Gets a secret available to a user's codespaces without revealing its encrypted value.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    pub async fn get_user_codespaces_secrets_secret_name<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretName(secret_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/secrets/{secret_name}/repositories`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-selected-repositories-for-a-user-secret>
    ///
    /// List selected repositories for a user secret
    /// List the repositories that have been granted the ability to use a user's codespace secret.
    /// You must authenticate using an access token with the `user` or `read:user` scope to use this endpoint. User must have Codespaces access to use this endpoint.
    pub async fn get_user_codespaces_secrets_secret_name_repositories<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretssecretNameRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretNameRepositories(secret_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/{codespace_name}`
    /// * docs <https://docs.github.com/rest/reference/codespaces#get-a-codespace-for-the-authenticated-user>
    ///
    /// Get a codespace for the authenticated user
    /// Gets information about a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn get_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacescodespaceNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * patch `/user/codespaces/{codespace_name}`
    /// * docs <https://docs.github.com/rest/reference/codespaces#update-a-codespace-for-the-authenticated-user>
    ///
    /// Update a codespace for the authenticated user
    /// Updates a codespace owned by the authenticated user. Currently only the codespace's machine type can be modified using this endpoint.
    ///
    /// Once you specify a new machine type it will be applied the next time your codespace is started.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn patch_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserCodespacescodespaceNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * get `/user/codespaces/{codespace_name}/machines`
    /// * docs <https://docs.github.com/rest/reference/codespaces#list-machine-types-for-a-codespace>
    ///
    /// List machine types for a codespace
    /// List the machine types a codespace can transition to use.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn get_user_codespaces_codespace_name_machines<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacescodespaceNameMachinesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceNameMachines(codespace_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * post `/user/codespaces/{codespace_name}/start`
    /// * docs <https://docs.github.com/rest/reference/codespaces#start-a-codespace-for-the-authenticated-user>
    ///
    /// Start a codespace for the authenticated user
    /// Starts a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn post_user_codespaces_codespace_name_start<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostUserCodespacescodespaceNameStartResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStart(codespace_name),
            query,
            body,
        )
        .await
    }
    /// * tags codespaces
    /// * post `/user/codespaces/{codespace_name}/stop`
    /// * docs <https://docs.github.com/rest/reference/codespaces#stop-a-codespace-for-the-authenticated-user>
    ///
    /// Stop a codespace for the authenticated user
    /// Stops a user's codespace.
    ///
    /// You must authenticate using an access token with the `codespace` scope to use this endpoint.
    pub async fn post_user_codespaces_codespace_name_stop<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostUserCodespacescodespaceNameStopResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStop(codespace_name),
            query,
            body,
        )
        .await
    }
    /// * tags users
    /// * get `/user/gpg_keys/{gpg_key_id}`
    /// * docs <https://docs.github.com/rest/reference/users#get-a-gpg-key-for-the-authenticated-user>
    ///
    /// Get a GPG key for the authenticated user
    /// View extended details for a single GPG key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn get_user_gpg_keys_gpg_key_id<T, V>(
        &self,
        gpg_key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserGpgKeysgpgKeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserGpgKeysgpgKeyId(gpg_key_id), query, body)
            .await
    }
    /// * tags apps
    /// * get `/user/installations`
    /// * docs <https://docs.github.com/rest/reference/apps#list-app-installations-accessible-to-the-user-access-token>
    ///
    /// List app installations accessible to the user access token
    /// Lists installations of your GitHub App that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
    ///
    /// You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
    ///
    /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
    ///
    /// You can find the permissions for the installation under the `permissions` key.
    pub async fn get_user_installations<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserInstallationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserInstallations(), query, body)
            .await
    }
    /// * tags apps
    /// * get `/user/installations/{installation_id}/repositories`
    /// * docs <https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-user-access-token>
    ///
    /// List repositories accessible to the user access token
    /// List repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access for an installation.
    ///
    /// The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
    ///
    /// You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
    ///
    /// The access the user has to each repository is included in the hash under the `permissions` key.
    pub async fn get_user_installations_installation_id_repositories<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserInstallationsinstallationIdRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserInstallationsinstallationIdRepositories(installation_id),
            query,
            body,
        )
        .await
    }
    /// * tags interactions
    /// * put `/user/interaction-limits`
    /// * docs <https://docs.github.com/rest/reference/interactions#set-interaction-restrictions-for-your-public-repositories>
    ///
    /// Set interaction restrictions for your public repositories
    /// Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.
    pub async fn put_user_interaction_limits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutUserInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutUserInteractionLimits(), query, body)
            .await
    }
    /// * tags users
    /// * get `/user/keys/{key_id}`
    /// * docs <https://docs.github.com/rest/reference/users#get-a-public-ssh-key-for-the-authenticated-user>
    ///
    /// Get a public SSH key for the authenticated user
    /// View extended details for a single public SSH key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
    pub async fn get_user_keys_key_id<T, V>(
        &self,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserKeyskeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserKeyskeyId(key_id), query, body)
            .await
    }
    /// * tags orgs
    /// * get `/user/memberships/orgs/{org}`
    /// * docs <https://docs.github.com/rest/reference/orgs#get-an-organization-membership-for-the-authenticated-user>
    ///
    /// Get an organization membership for the authenticated user
    ///
    pub async fn get_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserMembershipsOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserMembershipsOrgsorg(org), query, body)
            .await
    }
    /// * tags orgs
    /// * patch `/user/memberships/orgs/{org}`
    /// * docs <https://docs.github.com/rest/reference/orgs#update-an-organization-membership-for-the-authenticated-user>
    ///
    /// Update an organization membership for the authenticated user
    ///
    pub async fn patch_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserMembershipsOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUserMembershipsOrgsorg(org), query, body)
            .await
    }
    /// * tags migrations
    /// * get `/user/migrations/{migration_id}`
    /// * docs <https://docs.github.com/rest/reference/migrations#get-a-user-migration-status>
    ///
    /// Get a user migration status
    /// Fetches a single user migration. The response includes the `state` of the migration, which can be one of the following values:
    ///
    /// *   `pending` - the migration hasn't started yet.
    /// *   `exporting` - the migration is in progress.
    /// *   `exported` - the migration finished successfully.
    /// *   `failed` - the migration failed.
    ///
    /// Once the migration has been `exported` you can [download the migration archive](https://docs.github.com/rest/reference/migrations#download-a-user-migration-archive).
    pub async fn get_user_migrations_migration_id<T, V>(
        &self,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserMigrationsmigrationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserMigrationsmigrationId(migration_id),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/user/packages/{package_type}/{package_name}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-for-the-authenticated-user>
    ///
    /// Get a package for the authenticated user
    /// Gets a specific package for a package owned by the authenticated user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_user_packages_package_type_package_name<T, V>(
        &self,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserPackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageName(package_type, package_name),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/user/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-version-for-the-authenticated-user>
    ///
    /// Get a package version for the authenticated user
    /// Gets a specific package version for a package owned by the authenticated user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_user_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserPackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageNameVersionspackageVersionId(
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags users
    /// * get `/users/{username}/hovercard`
    /// * docs <https://docs.github.com/rest/reference/users#get-contextual-information-for-a-user>
    ///
    /// Get contextual information for a user
    /// Provides hovercard information when authenticated through basic auth or OAuth with the `repo` scope. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.
    ///
    /// The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository via cURL, it would look like this:
    ///
    /// ```shell
    ///  curl -u username:token
    ///   https://api.github.com/users/octocat/hovercard?subject_type=repository&subject_id=1300192
    /// ```
    pub async fn get_users_username_hovercard<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameHovercardResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUsersusernameHovercard(username), query, body)
            .await
    }
    /// * tags apps
    /// * get `/users/{username}/installation`
    /// * docs <https://docs.github.com/rest/reference/apps#get-a-user-installation-for-the-authenticated-app>
    ///
    /// Get a user installation for the authenticated app
    /// Enables an authenticated GitHub App to find the user’s installation information.
    ///
    /// You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
    pub async fn get_users_username_installation<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameInstallation(username),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/users/{username}/packages/{package_type}/{package_name}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-for-a-user>
    ///
    /// Get a package for a user
    /// Gets a specific package metadata for a public package owned by a user.
    ///
    /// To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_users_username_packages_package_type_package_name<T, V>(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernamePackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageName(
                username,
                package_type,
                package_name,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags packages
    /// * get `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}`
    /// * docs <https://docs.github.com/rest/reference/packages#get-a-package-version-for-a-user>
    ///
    /// Get a package version for a user
    /// Gets a specific package version for a public package owned by a specified user.
    ///
    /// At this time, to use this endpoint, you must authenticate using an access token with the `packages:read` scope.
    /// If `package_type` is not `container`, your token must also include the `repo` scope.
    pub async fn get_users_username_packages_package_type_package_name_versions_package_version_id<
        T,
        V,
    >(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(
                username,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/users/{username}/settings/billing/actions`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-actions-billing-for-a-user>
    ///
    /// Get GitHub Actions billing for a user
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    ///
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    ///
    /// Access tokens must have the `user` scope.
    pub async fn get_users_username_settings_billing_actions<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingActions(username),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/users/{username}/settings/billing/packages`
    /// * docs <https://docs.github.com/rest/reference/billing#get-github-packages-billing-for-a-user>
    ///
    /// Get GitHub Packages billing for a user
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `user` scope.
    pub async fn get_users_username_settings_billing_packages<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingPackages(username),
            query,
            body,
        )
        .await
    }
    /// * tags billing
    /// * get `/users/{username}/settings/billing/shared-storage`
    /// * docs <https://docs.github.com/rest/reference/billing#get-shared-storage-billing-for-a-user>
    ///
    /// Get shared storage billing for a user
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and Github Packages.
    ///
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    ///
    /// Access tokens must have the `user` scope.
    pub async fn get_users_username_settings_billing_shared_storage<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingSharedStorage(username),
            query,
            body,
        )
        .await
    }
}
